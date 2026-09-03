# Count 3 — a warn-by-default rustc lint inside a constitution fence

Two files in `xtask/src` make contradictory claims about the same mechanism.

`xtask/src/constitution.rs:31-34`:

> `RUSTDOCFLAGS=-D warnings` recovers rustc's *default-on* lints inside a
> doctest — a probe confirmed `non_snake_case` fails the build under it — but
> not the workspace's `[lints]` table and not clippy…

`xtask/src/narrative.rs:42-54`, recording a re-measurement on the pinned 1.97.1:

> a fence violating `non_snake_case`, a warn-by-default rustc lint confirmed to
> fire on the same snippet under plain `rustc`, compiled and ran with **no
> diagnostic and exit 0** — with the variable set, with it removed…

`narrative.rs`'s measurement was taken against the **narrative** tree. This one
is taken against the **constitution** corpus, which is the tree
`constitution.rs`'s own sentence is about, and against the gate step as
`REQUIRED` declares it.

## The edit

Two `#`-hidden doctest lines, inserted immediately after the first fence's own
hidden `fn main() {` in `standards/rust/00-prime-directives.md`
(`results/raw/fence-diff.txt`):

```diff
 # fn main() {
+# let notSnakeCase = 1u8;
+# assert_eq!(notSnakeCase, 1);
 #     fn assert_unpin<T: Unpin>() {}
```

*Hidden*, so the rendered fence is byte-identical and `lint-constitution` — the
`REQUIRED` step immediately before this one, which reads these files as text —
cannot be what answers. *Used*, so `unused_variables` cannot fire and confuse
the attribution: if anything goes red, `non_snake_case` is the only lint that
can have made it.

## Both controls fired

| control | expects | result | raw |
| --- | --- | --- | --- |
| the snippet under plain `rustc --edition 2024` | a diagnostic, or the arm is measuring a lint that was never going to fire | `warning: variable notSnakeCase should have a snake case name` … `= note: #[warn(non_snake_case)] (part of #[warn(nonstandard_style)]) on by default`, **exit 0** | `lint-control.txt` |
| a **type error** at the same insertion point | the docs step red, or a green result means only that the edit was never compiled | `test result: FAILED. 232 passed; 1 failed`, failing test named `00-prime-directives.md - constitution::prime_directives (line 32)`, **exit 101** | `fence-control-typeerror.txt` |

The second control is the one that makes the result below mean anything: the
edit reaches a compiler, at that exact line, and a compiler *does* refuse
something there.

## The result

| arm | command | result |
| --- | --- | --- |
| the step | `RUSTDOCFLAGS="-D warnings" cargo test --locked -p xtask --doc` | **exit 0**, `test result: ok. 62 passed; 0 failed` |
| the whole gate | `cargo xtask ci` | **exit 0**, all 33 steps |

`constitution.rs:31-34`'s parenthesis — *"a probe confirmed `non_snake_case`
fails the build under it"* — is false on the pinned toolchain.
`narrative.rs:42-54` is right, and its measurement generalises from the
narrative tree to the constitution corpus.

## What this costs, stated narrowly

`main.rs:736-739` justifies the step's `RUSTDOCFLAGS` as *"the whole of what the
constitution's examples are held to"*, on the strength of the claim above. What
they are actually held to is type-checking, plus `lint_constitution`'s
`FORBIDDEN_IN_FENCE` grep for `.unwrap()` and `.expect(`. Every rustc
warn-by-default lint — `non_snake_case` here, and by the same mechanism
`unused_mut`, `deprecated`, `unused_imports` — is unenforced inside every fence
in `standards/rust/`.

That matters more here than it would elsewhere, because the constitution's whole
enforcement model is that its examples are checked by the compiler rather than
by a reader, and within an atom **the compiled example beats the prose**
(`standards/rust/README.md`). The examples are still compiled; they are just not
linted, and one of the two files describing that mechanism says otherwise.
