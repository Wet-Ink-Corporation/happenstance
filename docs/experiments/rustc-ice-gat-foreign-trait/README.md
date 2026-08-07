# A rustc ICE that this architecture reaches by construction

A projection store that **borrows** its database rather than owning it does not
compile — correctly, because the store type then carries a lifetime and does not
outlive the one the trait method requires. rustc works that out and then crashes
while writing the error message.

```text
thread 'rustc' panicked at compiler\rustc_trait_selection\src\errors\note_and_explain.rs:27:22:
DefId::expect_local: `DefId(14:6 ~ tcrate[beda]::Store::commit)` isn't local

query stack during panic:
#0 [compare_impl_item] checking assoc item `<impl at icrate.rs:2:1: 2:37>::commit::{anon_assoc#0}` is compatible with trait definition
#1 [check_well_formed] checking that `<impl at icrate.rs:2:1: 2:37>` is well-formed
```

The panic message names the mechanism. The diagnostics code is trying to point at
the trait method to explain *where* the lifetime obligation came from, and it
assumes that item is in the crate being compiled. When the trait is foreign the
assumption is false and rustc panics on the spot — in the frames below the panic,
`report_region_errors` → `extern_crates_with_the_same_name`. **The bug is entirely
in the path that explains a region error, never in the one that detects it.**

## Why this lives in happenstance's documentation

Three reasons, and none of them is "we found a compiler bug".

**Every adapter is a foreign crate relative to `happenstance-core`.** That is the
architecture rather than an accident: the contract crate defines the ports and
adapters implement them from outside. The single ingredient that turns a clean
diagnostic into a compiler crash is therefore permanent and structural for anyone
writing an adapter against this workspace.

**`where Self: 'a` on `ProjectionStore`'s GAT is one of the five ingredients**,
and [phase 6](../../RUNBOOK.md#phase-6--freeze-projectionstore) decides whether
that GAT survives. If it goes, this exposure goes with it. That makes the table
below an input to a decision rather than trivia — which is the whole reason this
is a directory and not a bug report someone remembers.

**An adapter author who tries a borrowing store gets a compiler crash instead of
a diagnostic**, and the crash says nothing about the mistake. `happenstance-ladybug`
hit it while attempting exactly that; see
[`adapter-shapes.md`](../../adapter-shapes.md) §6 for where it sits among that
crate's three attempted batch shapes.

## The reproduction

Two files, no dependencies, no cargo, no `async`, no `Send`.

[`repro/tcrate.rs`](repro/tcrate.rs):

```rust
pub trait Store {
    type Batch<'a>
    where
        Self: 'a;

    fn commit(&self, batch: Self::Batch<'_>) -> impl Sized;
}
```

[`repro/icrate.rs`](repro/icrate.rs):

```rust
pub struct Borrowing<'a>(&'a ());

impl tcrate::Store for Borrowing<'_> {
    type Batch<'a>
        = ()
    where
        Self: 'a;

    fn commit(&self, _batch: Self::Batch<'_>) -> impl Sized {}
}
```

```console
$ cd repro
$ rustc --edition 2024 --crate-type lib tcrate.rs
$ rustc --edition 2024 --crate-type lib icrate.rs --extern tcrate=libtcrate.rlib -L .
```

The first succeeds. The second panics.

## What is load-bearing

Five ingredients, each independently necessary. Remove any one and the ICE
disappears — twice into a clean `E0477`, three times into code that compiles.

| removed | result |
|---|---|
| the crate split (trait moved in beside the impl) | clean `E0477`, no ICE |
| `where Self: 'a` on the GAT | compiles clean |
| the RPITIT return (plain `fn commit(&self, batch: Self::Batch<'_>);`) | clean `E0477`, no ICE |
| the impl self type's lifetime (`impl Store for Plain`) | compiles clean |
| the GAT's lifetime parameter (plain `type Batch;`) | compiles clean |

**`async` is not among them**, and that is the finding worth having. It appeared
necessary because `async fn` in a trait desugars to a method returning
`impl Future` — a return-position `impl Trait`. Any RPITIT does it; `-> impl Sized`
is enough. `Send` is not required either, as a supertrait or as a return bound,
and neither is the `&self` receiver, passing the batch by reference, a non-trivial
batch type, or edition 2024 — 2018 and 2021 reproduce as well.

The table is generated rather than remembered:

```console
$ ./bisect.sh                          # the pinned toolchain
$ ./bisect.sh 'rustup run 1.85 rustc'  # or any other
```

It prints the rows above. If it stops doing so, the compiler has changed under
this document, which is the thing worth being told.

## Version range

Reproduces on **1.85.1**, on **1.97.1** (the pinned toolchain), and on
**1.99.0-nightly (7608eb7b0)**, where the file has moved to
`rustc_trait_selection/src/diagnostics/note_and_explain.rs:27:22`. At 1.85 the
query stack says `{synthetic#0}` where later versions say `{anon_assoc#0}`.

So: **not a regression, and not fixed on nightly.** The bisection table above is
byte-identical at 1.85.1 and 1.97.1.

## Upstream

Reported as a comment on
[rust-lang/rust#158983](https://github.com/rust-lang/rust/issues/158983) —
[the comment](https://github.com/rust-lang/rust/issues/158983#issuecomment-5218463761).

That issue is open, carries `E-needs-mcve`, and arrives from a strikingly similar
place: a database-driver trait (`echorm`) with `where Self: 'a` GATs, RPITIT
futures, and `impl Connection for SqliteConnection<'_>`. Its existing summary says
*"Needs two crates, needs async"*; the contribution here is that `async` is
incidental and cargo is not needed at all.

Two related and previously uncross-referenced:
[#153375](https://github.com/rust-lang/rust/issues/153375), the same panic and
query stack, never minimised — that reporter independently found the crate
boundary load-bearing; and
[#153382](https://github.com/rust-lang/rust/pull/153382), an `expect_local` →
`as_local` fix closed unmerged after its regression test was shown never to enter
the diagnostic path.

A new issue was deliberately **not** filed.
