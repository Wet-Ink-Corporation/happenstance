# `spec-trace`'s prose guard is retired. What does its open question owe, and what do the 45 declarations that replaced it say?

Short answer up front: **the open question `kb-open-question-no-ps-rule-name-resolved-001`
is answered on two of its three sub-questions and its stated mechanism no longer
exists, so it needs closing rather than carrying; and the 45 declarations that
replaced the guard are a measurement the specification has never had, which the
`[FROZEN]` `PS` and `ES` clauses' owners should read before the next freeze.**

This brief records a change that has landed, not one being proposed. What it asks
for is a decision about the *record* left behind: an accepted open question whose
mechanism was deleted, and a number — 26 of the 45 unresolvable names are real
tests in files no resolution source reads — that nobody had before.

**This brief did not get the author → two-critic → revision pass the original
thirteen had.** It was written by the lane that made the change, in the same
session. Read it with that discount.

---

## Why this is owed

`.kb/open-questions/no-ps-rule-name-is-resolved.md`
(`kb-open-question-no-ps-rule-name-resolved-001`) is an **accepted** open question
whose "what is true today" section describes a mechanism this lane deleted. Two of
its citations now point at code that does not exist:

- `:58` cited `xtask/src/spec_trace.rs:699` for the loop guard
  `c.schedules_new || !has_suite(&c.id)`. The `schedules_new` disjunct is gone.
- `:60` cited `:1630-1634` for the expression that set `schedules_new`. The
  expression is gone; `Rules` no longer carries the field.

Neither can be repointed by anchor, because the anchor is what was removed. Every
other citation in that file was repointed by this lane (commit `b0e766f` and its
follow-up); these two were deliberately left, because moving them to something
adjacent would assert a mechanism the file describes and the tree no longer has.

The lane is explicitly **not** the owner of an open question and may not settle
one, so this brief records the state rather than editing the atom.

---

## What is true today

### 1. Two of the open question's three sub-questions are measured

Measured in this working tree at `9b06836`, by instrumenting `rules_of` and
printing per-term firings over the real `spec/SPECIFICATION.md`:

| Term in the guard | Clauses it fired on (all families) |
|---|---|
| `(new)` | 9 |
| `†` (a bare dagger in the `Rule:` line) | **0** |
| a leading `new ` | **0** |
| `` new ` `` | 10 |
| `unit test` | 6 |
| `compile test` | 5 |
| `meta-test` | 8 (all `CF`, none with a suite) |

**Sub-question 1** — *is the dagger doing work `has_suite` cannot?* — no. It fires
on nothing. The file carries 59 daggers and every one is inside §7's generated
table or the prose describing the convention.

**Sub-question 2** — *if the dagger is retired, how many of the seventeen daggered
`PS` clauses newly fail?* — zero, which is what
`experiments/gate-vacuity/results/raw/spec-trace-drop-dagger.txt` already recorded
and what this measurement independently reproduces.

The open question's own stated mechanism — *"`schedules_new` is set by a bare
dagger in the clause's `Rule` line"* — was **already wrong** at `56ef6c5`, as the
review's `S-5` says. The term doing the work on the `PS` family is `` new ` ``.

**Sub-question 3** — *is the dagger convention superseded by the maturity
markers?* — is untouched here and is an ADR's, not this brief's.

### 2. What the guard was hiding, now enumerated

Retiring the guard makes check 4 resolve every rule name a clause cites. 45 names
across 28 clauses resolve against nothing, and `UNRESOLVABLE_RULE_NAMES` now
carries each with a reason:

| Kind | Count | What it means |
|---|---|---|
| `Elsewhere(path)` | **26** | The test exists, in a file no resolution source reads. The path is opened on every run and the identifier must still be there. |
| `NotARuleName` | **4** | `trait_variant` (ES-2), `spec_trace` and `compile_fail` (WF-12), `compile_fail` (VT-32) — harvested out of prose by `backticked_idents`. |
| `Scheduled(who)` | **15** | Nothing has written it. |

The 26 are the number worth having. Twelve `[FROZEN]` clauses — VT-13, VT-18,
VT-20, VT-26, VT-32, VT-33, WF-12, ES-2, ES-3, ES-4, ES-5 and (via VT-10) the
foreign-identity target — stand on tests that `collect_rules` has never read,
because they live in `happenstance-core`'s `src/` and `tests/`, or in the
testkit's `tests/` and `lib.rs`. Before this change nothing in the workspace would
have noticed one being deleted. It does now, but by a **path in a table**, which
is a weaker instrument than resolution.

### 3. The 15 scheduled ones are not one kind

Nine are the replication and projection families waiting on suites that do not
exist. One is not:

> ES-6 `store_error_crosses_a_join_handle` — nothing, yet:
> `crates/happenstance-cloudflare/src/send_shape.rs` argues it is **unwritable
> against today's port**, which is a finding rather than a schedule.

`.kb/open-questions/es-6-names-an-unwritable-rule.md` already carries that one.

---

## The question, and the options

**Q1. What happens to `kb-open-question-no-ps-rule-name-resolved-001`?**

- **Option A — close it.** Its sub-questions 1 and 2 are measured, and the
  mechanism its "what is true today" describes is gone. Cost: sub-question 3 (the
  dagger convention versus the maturity markers) loses its home and needs a new
  one.
- **Option B — supersede it with a narrower atom carrying only sub-question 3.**
  Cost: one more atom; benefit: the two dangling citations go with the superseded
  body rather than being edited in place, which is the discipline `CLAUDE.md`
  sets for decision atoms and which this repository applies to open questions by
  convention.
- **Option C — leave it.** Cost: an accepted atom describing a mechanism that
  does not exist, with two citations pointing at deleted code that `cargo xtask
  lints`' V-6 does not fail on, because both land on plausible non-blank lines.

**Recommendation: B.** The strongest argument against it is that superseding an
open question for a *mechanism* change is heavier than the change deserves — the
question ("does any `PS` rule name get resolved?") is answered by `has_suite`
admitting `PS`, which happened in the 2026-08-15 wave, not here. Against that:
the atom's body is now three paragraphs of description of code that was deleted,
and a reader who trusts it will look for `schedules_new` and not find it.

**Q2. Do the 26 `Elsewhere` declarations become resolution sources instead?**

Widening `resolvable` to include `happenstance-core/src/*.rs`'s `#[test]` items
would turn 26 declarations into 26 resolved names — a stronger instrument. It is
**not** taken here for one mechanical reason: `resolvable` also feeds §7.2's
rendering, so widening it moves the generated table, and §7.2's committed region
would have to be regenerated in `spec/SPECIFICATION.md`. That file is held by
another lane. It is a clean follow-up for whoever holds it next.

---

## Cost of delay

Low for Q1 and it does not expire. Q2 costs one thing that compounds: every
`Elsewhere` entry is a path, and a path is repointed by hand when a file moves.
Twenty-six of them is a maintenance surface that resolution would not have.

---

## What this does not settle

Whether the dagger convention is superseded by the maturity markers (sub-question
3). Whether `backticked_idents` should stop harvesting the four `NotARuleName`
tokens at the parser rather than declaring them — declaring them was chosen
because a parser-side filter is a second place to state which tokens are not
rules and would silently swallow a real rule name matching its shape.
