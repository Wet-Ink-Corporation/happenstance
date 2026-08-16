# Phase 7 — is `happenstance-macros` in scope for 0.1?

- **Date:** 2026-08-16
- **Pinned to:** `78a2170c1d06bad5eec34915b0b3682f524ec91f`
- **Produced by:** HS-P0011 *Typed layer and alpha release*, story
  `defect-log-and-macros-verdict` (HS-S0032), discharging project **AC-013** and
  `RUNBOOK.md`'s phase-7 exit criterion.
- **Lifecycle:** immutable evidence. Superseded rather than edited.

**Verdict: OUT.** The rewritten example does **not** carry more mapping
boilerplate than domain logic. It carries less, at both extremes of the one
contested block, and by a wide margin in both. This **contradicts** the
signed-off design's recorded prediction, which is the outcome that prediction
was written down to make possible.

The rest of this document is the evidence, in the order it was produced: the
prediction, then the counting method, then the count, then the verdict. It is
arranged that way on purpose — a record that states its conclusion first invites
a reader to check the arithmetic against the conclusion instead of the other way
round.

---

## 1. The prediction, recorded before the measurement

`RUNBOOK.md:525` sets the criterion in one sentence:

> Is `happenstance-macros` in scope for 0.1 · phase 7 · *open — the criterion is
> stated in phase 7 and evaluated in its session log*

and phase 7's exit box (`RUNBOOK.md:4079-4083`) states it as a test:

> *if the rewritten example carries more mapping boilerplate than domain
> logic, the derive is in scope for 0.1.* Record the answer either way.

The signed-off design went further and predicted the answer, with a number
(`_design.md:1103-1111`):

> Domain logic — the enum, the fold's two arms, the decision — is **11 lines**.
> Mapping ceremony — `EVENT_TYPES`, `event_type`, `tags`, `encode`,
> `decode` — is **26 lines**. That is 2.4:1 against the domain for a
> two-variant enum, and it will worsen, not improve, with a third. **The
> prediction this design records: AC-013's verdict is
> "`happenstance-macros` is in scope for 0.1"** … That
> prediction is falsifiable and must be checked against the *rewritten example*,
> not against this doctest.

That last sentence is why this document exists and why the substrate is what it
is. The **prior** is 26 : 11, or **2.36 : 1** ceremony-to-domain, over the
design's own doctest. It is recorded here as a prior and is **not** the
criterion.

---

## 2. The counting method

Stated in full so a second reader can re-run it and reach the same two integers.

**Substrate.** `examples/course-subscriptions/src/main.rs` as
`worked-example-on-typed-layer` left it, at the pinned commit. 532 lines, `wc
-l`. It is the example and not the doctest, per `_design.md:1110-1111`.

**Substrate validity, checked before counting** (EC-005). That story's NF-002
requires the mapping to be written **plainly** — no local `macro_rules!`, no
helper trait, no blanket impl that shrinks the `DomainEvent` impl — precisely
because this measurement reads the file. Verified at the pinned commit: the file
contains no `macro_rules!`, and `impl DomainEvent for Enrolment` (`:209-247`) is
written out by hand, variant by variant. The substrate is valid and the count
was taken.

**Partition.** Every one of the 532 lines is assigned to exactly one of four
buckets. Blank lines and comment-only lines follow the block they document. The
ranges are contiguous, non-overlapping, and exhaust the file — which is the
check that makes the totals re-derivable rather than asserted.

| Bucket | Definition |
| --- | --- |
| **ceremony** | What a derive would emit: `const EVENT_TYPES`, `event_type()`, `tags()`, and the encode/decode plumbing. (`assert_domain_event` is named by the method too; the example does not call it, so it contributes nothing here) |
| **domain** | The event enum's variants and payloads, each model's state, its `scope`, its `apply` arms, and each decision function's body |
| **neither** | `main`'s I/O, the transcript printing, store construction, imports, and the call-site plumbing around `commit(...)` — which no derive emits and which is not a decision |
| **contested** | Genuinely both, reported at both extremes rather than assigned (EC-007) |

**Threshold.** *More* mapping boilerplate than domain logic means `ceremony /
domain > 1`. Exactly 1.0 is **out** (EC-006).

**Two judgement calls, stated because they move the number.**

1. **The `commit(...)` scaffolding inside each handler is `neither`, not
   domain.** The `commit(`, `store,`, `boundary,`, `Retry::attempts(…)`, `)`,
   `.await`, `.map`, `.map_err` lines are call-site plumbing. They are not a
   decision, and no derive emits them. This **shrinks** domain by 30 lines,
   which moves the ratio *toward* "in" — against the verdict below, which is the
   direction a judgement call should err in.
2. **`CourseId` and `StudentId` are contested, not assigned.** By the method's
   letter they are *"the event enum's … payloads"*, which is domain. But their
   `tag` field, their `TryFrom<String>`/`Into<String>` serde bridge and their
   constructors exist **only** because `DomainEvent::tags` is infallible while
   `Tags` has no infallible constructor — the file says so at `:104-109`, and
   that is defect **D-2** in `phase-7-contract-defects.md`. A richer derive
   supporting `#[tag("course")]` on a plain `String` field would delete most of
   them. Eighty-one lines is far too much to decide by fiat, so they are
   reported at both extremes.

---

## 3. The count

One row per contiguous range. Totals are **derived by summing the rows**, not
asserted beside them.

| Lines | n | Bucket | What it is |
| --- | ---: | --- | --- |
| 1–29 | 29 | neither | Crate-level narration: the three invariants, why DCB dissolves them, how to run it |
| 30–31 | 2 | neither | `#![allow(clippy::print_stdout)]` — enables the transcript |
| 32–39 | 8 | neither | Imports |
| 40–48 | 9 | domain | `ATTEMPTS` and its doc. A retry bound is a decision the application takes; no derive emits it |
| 49–97 | 49 | neither | `main`: store construction, the seven transcript sections, the final-log loop |
| 98–101 | 4 | **contested** | The identity section banner, following the block it documents |
| 102–182 | 81 | **contested** | `CourseId` and `StudentId`: the payload newtypes, their `Tag` field, constructors and serde bridge |
| 183–186 | 4 | domain | The domain section banner |
| 187–208 | 22 | domain | `Enrolment`'s doc and its three variants with their payloads |
| **209–248** | **40** | **ceremony** | `impl DomainEvent for Enrolment`: `EVENT_TYPES` (`:210-214`), `event_type` (`:216-222`), `tags` (`:224-234`), `encode` (`:236-238`), `decode` (`:240-246`) |
| 249–295 | 47 | domain | `Refusal`: five variants, each carrying the value its message prints |
| 296–299 | 4 | domain | The decision-models section banner |
| 300–334 | 35 | domain | `CourseDefinition`: state, `new`, `scope`, `apply` |
| 335–373 | 39 | domain | `Seats`: state, `new`, `scope`, `apply` |
| 374–415 | 42 | domain | `StudentSeat`: state, `new`, `scope`, `apply` |
| 416–419 | 4 | neither | The handlers section banner |
| 420–424 | 5 | neither | `define_course` doc, signature, edge validation |
| 425–428 | 4 | neither | `commit(...)` call scaffolding |
| 429–439 | 11 | domain | `define_course`'s decision closure |
| 440–445 | 6 | neither | `.await` / `.map` / `.map_err` tail |
| 446–458 | 13 | neither | `subscribe` doc, signature, edge validation |
| 459–462 | 4 | neither | `commit(...)` call scaffolding |
| 463–486 | 24 | domain | `subscribe`'s decision closure — the three-refusal, two-model decision |
| 487–492 | 6 | neither | Tail |
| 493–498 | 6 | neither | `unsubscribe` doc, signature, edge validation |
| 499–502 | 4 | neither | `commit(...)` call scaffolding |
| 503–514 | 12 | domain | `unsubscribe`'s decision closure |
| 515–520 | 6 | neither | Tail |
| 521–532 | 12 | neither | `rejected` — renders a command failure as the line the transcript prints |

**Partition check.** The ranges are contiguous from 1 to 532 with no gap and no
overlap. Summing the `n` column gives **532**, which equals `wc -l
examples/course-subscriptions/src/main.rs` at the pinned commit.

**Totals, derived from the rows.**

| Bucket | Lines |
| --- | ---: |
| ceremony | **40** |
| domain | **249** |
| neither | **158** |
| contested | **85** |
| *total* | *532* |

---

## 4. The verdict

Reported at both extremes of the contested block, per EC-007.

| Assignment of the contested 85 | ceremony | domain | ratio | verdict |
| --- | ---: | ---: | ---: | --- |
| all contested counted as **ceremony** | 125 | 249 | **0.50 : 1** | **out** |
| all contested counted as **domain** | 40 | 334 | **0.12 : 1** | **out** |

**The two extremes agree, so the verdict stands and the contested set is a
footnote.** The answer is not close to the threshold at either end: even the
assignment most favourable to a derive leaves the domain twice the size of the
ceremony.

**Verdict: `happenstance-macros` is OUT of scope for 0.1.**

### The prediction is contradicted, by name

`_design.md:1104-1111` predicted **in**, at 2.4 : 1. The example returns at
worst 0.50 : 1 — a factor of roughly five the other way. The design wrote that
number down as falsifiable so that this measurement could falsify it, and it
has.

**Why the two disagree, since a reader will want to know before trusting
either.** The doctest is a *minimum viable domain*: one two-variant enum, an
empty `Tags`, a two-arm fold, no refusal vocabulary, no decision models and one
call site. The `DomainEvent` impl is a **fixed cost** that barely grows with the
domain — 26 lines for two variants there, 40 for three variants here — while the
domain grows with the number of consistency concerns, refusals and handlers,
which is what an application actually has. So the ratio is a function of how
much domain the artefact contains, and the doctest measures it at the one point
where there is almost none.

The design's own "it will worsen with a third variant" was right about the
direction of the *ceremony* and wrong about the ratio: three variants cost 14
more lines of ceremony, and the domain they serve cost 200 more.

### What a derive would actually buy

**40 lines of 532 — 7.5% of the file.** It would delete `impl DomainEvent for
Enrolment` and nothing else. It would not touch the three decision models, the
five refusals, the three handlers, or the transcript. Against that it would cost
a fourth published crate, a proc-macro dependency in every consumer's build
graph, and a second way to express something the trait already expresses by hand
— at a phase whose whole purpose is to publish a small, honest surface.

**The one thing that could change the answer**, recorded so a later phase can
test it rather than rediscover it: a derive that also handled **tags from
runtime values** would reach into the contested 85 lines, not just the 40. That
is a materially bigger derive than the one this criterion asked about, and its
precondition is defect **D-2** being settled — `DomainEvent::tags` is infallible
over a type with no infallible constructor, which is what forces `CourseId` and
`StudentId` to exist at all. If D-2's decision record produces an infallible
`Tags` path, the identity newtypes shrink and this measurement should be
re-taken. **It would still be a post-0.1 question**: the count above is not
close enough for a 85-line swing to reach the threshold.

### Escalation

None is owed. `_decomposition.md:428` and `_storymap.md:155-157` route an **in**
verdict to the runbook as a scope change; this is an **out** verdict, so no new
workspace member is proposed and `crates/happenstance-macros/` is not created.
`publish-0-2-0-alpha-1` depends on this record, never on a crate, and is
unblocked by it.

The runbook is updated where it says the verdict is written: the phase-7 session
log cites this document, the exit box is discharged, and `RUNBOOK.md:525`'s
decision-table row moves off `open`.

---

## 5. Both altitudes, side by side

`_decomposition.md:307-312` asks for AC-U01 and AC-013 to be answered together,
because they are the same measurement read at two altitudes.

| Altitude | Artefact | Ceremony | Domain | Ratio | Reads as |
| --- | --- | ---: | ---: | ---: | --- |
| **Prior** (AC-U01) | `_design.md`'s first-program doctest, 2 variants | 26 | 11 | 2.36 : 1 | in |
| **Criterion** (AC-013) | `examples/course-subscriptions/src/main.rs`, 3 variants, 3 models, 3 handlers | 40 | 249 | 0.16 : 1 | **out** |
| **Criterion**, contested as ceremony | as above | 125 | 249 | 0.50 : 1 | **out** |

The prior is the honest worst case for a library's ergonomics and it is worth
keeping: **a reader's first program really does pay 2.4 : 1**, and that is what
AC-U01 measures and what the first-program page should keep being held to. It is
simply not what AC-013 asked, and the design said so in the same paragraph that
supplied the number.

---

## 6. Reproducing this

At `78a2170`:

```console
$ wc -l examples/course-subscriptions/src/main.rs
532
$ rg -c 'macro_rules!' examples/course-subscriptions/src/main.rs      # substrate check
0
$ sed -n '209,247p' examples/course-subscriptions/src/main.rs         # the ceremony, in full
```

Then walk the table in §3, sum the `n` column, and confirm it reaches 532. A
reader who disagrees with a row disagrees with one range and one reason, and can
recompute the totals without redoing the work — which is the whole point of
publishing the classification rather than the number.
