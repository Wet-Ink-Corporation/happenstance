# `experiments/suite-against-wrong-adapters`

Four wrong event store adapters, driven through the **whole** DCB conformance
suite, so that a pre-publication review can quote a number instead of an
argument.

**The question, and it is the only one this crate answers:**

> How many of the four wrong implementations the review named pass the whole
> conformance suite today?

**The answer, at `56ef6c5`: four of four. Every one of them is certified by all
89 rules.**

This is not a crate anybody depends on. It is **not a workspace member** (its
`Cargo.toml` carries an empty `[workspace]` table, the same trick
`experiments/append-condition/` and `experiments/wire-format/` use), it appears
in no `verify:` command and no `cargo xtask ci` step, and it adds no dependency
to any workspace manifest — `Cargo.lock` at the repository root is untouched.

## Why it exists

Three of the six findings it settles were arguments about what the rule set does
**not** cover, and an argument of that shape is exactly the kind this repository
refuses to accept on reasoning alone
(`standards/rust/01-standard-of-evidence.md`). Each of them names a wrong
implementation and asserts that the suite admits it. That assertion is
mechanically checkable, and until it is checked the reviewer and the maintainer
are exchanging opinions about a rule set that could simply be run.

It is a falsifier by construction. Had all four been rejected, three should-fix
findings would have collapsed and the theme would have lost its release
position. They were not.

## Conditions

| | |
| --- | --- |
| Machine | 13th Gen Intel Core i9-13905H, 20 logical cores, 32 GB RAM |
| OS | Windows 11 Home (10.0.26200) |
| Filesystem | NTFS, local NVMe |
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, `x86_64-pc-windows-msvc`; `cargo 1.97.1 (c980f4866 2026-06-30)` |
| `RUSTFLAGS` | `-D warnings`, inherited from `../../.cargo/config.toml` — cargo config discovery walks **up** across workspace boundaries, so an unused import fails this build too. It is not overridden: `RUSTFLAGS=""` *replaces* rather than merges and would drop the policy |
| Build | census `--release`; the two control binaries debug (they assert, they do not measure) |
| Repository commit | `56ef6c5ddc3476a9a896dfe1824f4c46de33a3ed` |
| Suite under measurement | `happenstance-testkit` 0.2.0-alpha.1, by path |
| Command | `./run.sh` |
| Wall clock | **117 s** for the whole of `run.sh` after `cargo clean` — i.e. from nothing, including the debug and release builds of `happenstance-core`, `happenstance-testkit` and this crate. The three test binaries themselves report 0.02 s, 0.00 s and 0.01 s |
| No I/O, no clock, no network | there is nothing here for a machine to be faster or slower at, which is why this README states a machine and then never quotes a duration as a result |

`results/raw/conditions.txt` is written by `run.sh` itself and carries the
toolchain, the commit and the ambient `rustflags` **read back off the files that
set them**, rather than restated here from memory.

## The subjects

Eleven, run through all 89 rules of the event-store family each. Four are the
measurement; six are controls in two directions; one is a diagnostic.

### The four under measurement

| # | Store | Finding | The defect, in one line |
|---|---|---|---|
| a | `ForwardPagingBudgetStore` | **L1-1** | forwards, `limit` is applied only when `from` is absent; the resume branch returns the whole tail. Backwards is left correct |
| b | `NoopReopenFixture` | **L1-2** | declares `REOPEN: Capability::SUPPORTED` over a `Vec`, and honours it with `async fn reopen(&self) {}` |
| c | `SwallowedReadFaultStore` | **L3-01** | `let Ok(page) = fetch().await else { return Poll::Ready(None) };` — a failed page fetch is reported as the end of the stream |
| d | `StagedCommitStore` | **F2-5** | genuinely suspends inside `append` between two statements, and commits atomically at the end |

(d) is the odd one and is labelled as such throughout: it is not a *wrong*
implementation. It is the **right** implementation of the shape the workspace
does not have — an interactive transaction, one round trip per row — and it
exists because F2-5's sharpened claim is that ES-22's `landed == 0` arm has never
executed against any store in the tree. See `results/what-the-number-settles.md`.

### The controls, in two directions

**Direction one — each mutant is otherwise a working store.** Every store is
generic over `const INJECTED: bool`, and the `true` arm carries a second,
unrelated defect on top: the testkit's own `InnerJoinTagStore` shape, untagged
events dropped by the read filter. Each injected arm **must** fail at least one
rule, and `tests/census.rs` asserts it. Without this, "fails nothing" is equally
consistent with a fixture the harness silently declined, a `connect` that handed
back an empty log, or a `const` that turned the rules off — none of which is a
statement about the suite. A store that is broken everywhere fails rules for the
wrong reason and would flatter the instrument rather than the bar.

**Direction two — the baseline is not itself broken.** `MemoryEventStore` (the
testkit's *published* reference fixture, registered unchanged) and `LogStore`
(the correct core the four are one step away from) must fail **nothing** in the
same run. `tests/census.rs` asserts that too.

Both assertions are real. If either fails, `cargo test` goes red and the number
is discarded rather than reported.

### The diagnostic

`SwallowedReadFaultStore (fault armed by hand)` separates two readings that
"fails nothing" would otherwise merge — *the suite cannot see this class of
defect at all* versus *the suite has no way to make it happen*. Only the second
is true. See `results/what-the-number-settles.md`.

## Conformance before the count

`run.sh` runs `tests/controls_are_conformant.rs` **first**, and it is not
ceremony. `tests/census.rs` reaches the rule set through the testkit's public
`for_each_event_store_rule!` enumeration and a probe harness of this crate's own,
because a failing rule has to become *data* rather than a dead binary — and that
harness could be subtly wrong in ways that produce exactly the output this
experiment is looking for. A rule that never ran, a fixture never constructed, a
panic caught in the wrong place: all three read as "the store passed".

So the two controls are also mounted through `event_store_conformance!` — the
one-line macro `CLAUDE.md` says an adapter must invoke before it is considered to
exist — as 178 ordinary `#[tokio::test]`s with no harness of this crate's in the
path. If those are red and the census's controls are green, the census is wrong.

`tests/defect_is_real.rs` is the second precondition and runs next: each of the
four defects is driven **through the port**, with no rule anywhere near it, and
the wrong observable behaviour is asserted. A strawman whose defect cannot be
seen through the port would pass the suite for the same reason a correct store
does, and counting it would prove nothing.

## What is copied, and what is not

`src/correct.rs` is copied **verbatim** from
`crates/happenstance-testkit/tests/mutation_coverage/correct.rs`, with two
mechanical edits recorded in its own header (`pub(crate)` → `pub`, and one
intra-doc link to a type that does not exist here demoted to prose). It is copied
rather than depended on because `tests/` is not a library — there is no way to
`use` the testkit's mutation-coverage primitives from outside its own test
binary — and copying is what makes the comparison honest: every store here is the
*repository's own* correct core with one step changed, exactly as the testkit's
forty registered mutants are.

`tests/support/harness.rs` is **adapted**, not copied, and reaches nothing an
adapter author outside this repository could not reach: `for_each_event_store_rule!`,
`rules::*`, `block_on`, `RuleOutcome`, `Capability`, `Fixture` — every one of
them a published item.

## Layout

```
src/correct.rs                    the repository's own correct core, copied
src/stores.rs                     the four stores, one step from it each
tests/support/harness.rs          rule-by-name × store-by-type, panics → data
tests/support/fixtures.rs         the eleven subjects
tests/controls_are_conformant.rs  the two controls, through event_store_conformance!
tests/defect_is_real.rs           each defect, through the port, with no rule involved
tests/census.rs                   THE MEASUREMENT
results/raw/                      untouched command output
results/census.md                 the table, written by hand from results/raw/
results/what-the-number-settles.md               the verdict on each of the six findings
```

## `run.sh` is not a gate step and must never become one

CF-34: a measurement harness is not part of the conformance bar, and a
measurement that can turn a merge red teaches people to re-run until green.
Nothing in `.redkiln/config.yaml` or `cargo xtask ci` invokes this, and the empty
`[workspace]` table means cargo cannot see the crate from the repository root at
all.

There is deliberately **no watchdog**, for CF-33's reason: a wall-clock deadline
inside the instrument is a flake inside the instrument the first time a runner is
loaded. The one store here that suspends inside `append` signals its waker
*before* returning `Pending`, so a rule waiting on it is slow rather than hung —
the mitigation is structural, which is the same answer
`tests/mutation_coverage/harness.rs` gives one crate over.

## Reproducing

```console
cd experiments/suite-against-wrong-adapters
./run.sh
```

The answer is the `ANSWER:` line at the end of `results/raw/census.txt`.
