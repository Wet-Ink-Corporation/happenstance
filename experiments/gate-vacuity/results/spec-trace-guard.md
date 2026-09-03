# Count 2 — the seven substrings that switch off a clause's rule-name check

`xtask/src/spec_trace.rs:699` is check 4 — *does the rule this clause names
exist?* — and it abstains outright:

```rust
if c.schedules_new || !has_suite(&c.id) {
    continue;
}
```

`schedules_new` (`:1630-1634`) is true if the clause's `Rule:` **text** contains
any of seven substrings: `(new)`, `†`, a leading `new `, `` new ` ``, or — via
`elsewhere` (`:1628-1629`) — `unit test`, `compile test`, `meta-test`.

## What the edit was, and what it deliberately was not

Only `schedules_new` is rewritten. **`elsewhere` is left computing exactly what
it computed before**, because it is read on its own by `rule_cell`
(`:1158`) and stored on the row (`:1383`); dropping a term from it would change
the §7.2 rendering as well, and the delta would no longer be attributable to
check 4. The three `elsewhere` terms are therefore *inlined* into
`schedules_new` as their own `text.contains(…)` calls and dropped from there.
`results/raw/` confirms the isolation: no run reports a stale §7.1–§7.2 region,
so nothing but check 4 moved.

## The count

| run | `cargo xtask spec-trace` | problems |
| --- | --- | ---: |
| unedited | exit 0, *"traceability: no problems found; §7.1–§7.2 matches the checker"* | 0 |
| all seven terms dropped | exit 1, *"45 traceability problem(s)"* | **45** |

Every one of the 45 is a check-4 line of the form *"`<clause>` names rule
`<name>`, which is not in `crates/happenstance-testkit/src/suite.rs` and the
clause does not declare it new"*. They fall across **28 distinct clauses**:

| family | clauses | rule names | clauses, by name |
| --- | ---: | ---: | --- |
| ES | 10 | 12 | ES-2, ES-3, ES-4, ES-5, ES-6, ES-29, ES-31, ES-38, ES-39, ES-40 |
| VT | 11 | 24 | VT-5, VT-6, VT-9, VT-10, VT-13, VT-14, VT-18, VT-20, VT-26, VT-32, VT-33 |
| PS | 6 | 6 | PS-25, PS-26, PS-27, PS-28, PS-29, PS-30 |
| WF | 1 | 3 | WF-12 |
| **total** | **28** | **45** | |

`CF` and `SY` contribute nothing, and not because they are clean: `has_suite`
(`:1750-1755`) admits only `ES-`, `VT-`, `WF-` and `PS-`, so a `CF` clause's
rule name is never resolved by check 4 whatever its `Rule:` line says. The
guard is the *second* reason a clause escapes check 4; the family prefix is the
first.

## Maturity of the 28

| marker | clauses |
| ---: | --- |
| **19 `[FROZEN]`** | ES-2, ES-3, ES-4, ES-5, ES-6, ES-29, ES-31, ES-38, PS-26, PS-28, PS-29, VT-5, VT-13, VT-18, VT-20, VT-26, VT-32, VT-33, WF-12 |
| 6 `[PROVISIONAL]` | ES-40, PS-25, VT-6, VT-9, VT-10, VT-14 |
| 3 `[DEFERRED]` | ES-39, PS-27, PS-30 |

A `[DEFERRED]` clause naming a rule nobody has written is the guard working as
designed — ES-39's `Rule:` line says `a_store_reports_the_history_it_does_not_hold`
**(new)**, *"writable only once the primitive is chosen"*, which is exactly true.
Nineteen `[FROZEN]` clauses in the same list is the finding.

## Per term

Each term dropped **alone**, so the number is what that term and nothing else is
holding back.

| term | clauses | rule names | exit |
| --- | ---: | ---: | ---: |
| `unit test` | 5 | **19** | 1 |
| `compile test` | 4 | **7** | 1 |
| `(new)` | 9 | **9** | 1 |
| `` new ` `` | 9 | **9** | 1 |
| `meta-test` | 0 | **0** | 0 |
| `†` | 0 | **0** | 0 |
| leading `new ` | 0 | **0** | 0 |

**Three of the seven guard nothing at this commit.** The finding's own
correction — that no clause `Rule:` line carries a `†`, so the dagger term is
inert — is confirmed and is not the whole of it: `meta-test` and the leading
`new ` are inert too, for the same kind of reason (`meta-test` appears only in
`CF` clauses, which `has_suite` already excludes). Three terms that cannot fire
are three terms whose removal is free.

The four live terms sum to 44, not 45, and the missing row is the one that shows
the terms are not independent: **VT-13** carries *both* `unit test` and
`` new ` `` on one `Rule:` line, so its `position_next_signals_overflow` is
reported only when both are gone.

```
`Rule:` unit test `position_next_signals_overflow`; `read_from_is_inclusive`,
`condition_after_ignores_events_at_the_boundary`; new
`read_from_a_gap_position` (ES-9's name for it)
```

That line also shows what the guard costs at its worst: two of the four names on
it — `read_from_is_inclusive` and `condition_after_ignores_events_at_the_boundary`
— *are* live suite rules and resolve fine once the guard is off. One prose word
switches off the check for all four.

## The open question's sub-question 2, answered

`.kb/open-questions/no-ps-rule-name-is-resolved.md` (accepted) asks, second in
its ordered list:

> If the dagger is retired from the guard, how many of the seventeen daggered
> `PS` clauses newly fail, and does that number change the answer to
> sub-question 1?

**Zero.** `results/raw/spec-trace-drop-dagger.txt` is exit 0 with no problems.
Retiring `text.contains('†')` from `schedules_new` changes nothing at all,
because at `56ef6c5` **no clause `Rule:` line contains a dagger** — `grep -c
"Rule:.*†" spec/SPECIFICATION.md` is 0, against 59 daggers in the file, all of
them in §7's generated table or in the prose describing the convention.

Two things follow, and they change the shape of the remediation rather than its
direction.

1. The atom's stated mechanism does not hold at this commit. Its summary says
   *"`schedules_new` is set by a bare dagger in the clause's `Rule` line, so
   every `PS` clause marked with a dagger … is skipped before its rule name is
   looked up."* The skipping is real and this experiment measures it — six `PS`
   clauses newly fail when the guard goes — but the term doing it is
   `` new ` ``, not `†`. Sub-question 1 ("is the dagger doing work `has_suite`
   cannot?") therefore has a measured answer too: no, it is doing no work at
   all.
2. The "seventeen" is not the number at `56ef6c5` either. §7.2 prints **6**
   daggered `PS` rows today (9 `ES`, 32 `SY`, 1 `CF`), and the dagger there is
   rendered by `rule_cell` off `rule_elsewhere`, which this experiment did not
   touch — a different mechanism from the guard, in a different function.

## Four of the 45 are not rule names at all

Removing the guard is not free, and this is the part a remediation has to answer
first. `backticked_idents` takes every backticked snake-case identifier on the
`Rule:` line, and four of the 45 are prose:

| clause | "rule" | what it actually is |
| --- | --- | --- |
| ES-2 | `trait_variant` | the proc-macro crate, named in the clause's reasoning |
| VT-32 | `compile_fail` | a doctest fence annotation |
| WF-12 | `compile_fail` | the same |
| WF-12 | `spec_trace` | the checker itself |

So the honest reading of 45 is **41 rule names a reader would expect to resolve,
plus 4 false positives the parser would need fixing to avoid** — which is why
the open question this corroborates
(`kb-open-question-no-ps-rule-name-resolved-001`) asks for the per-family count
before deciding whether the guard can go at all. That number is now measured.
