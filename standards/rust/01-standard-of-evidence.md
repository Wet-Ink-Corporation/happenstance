# 01 — The standard of evidence

> **Load when:** adding or editing a rule in `standards/rust/` · reaching for a
> `compile_fail` fence to prove something · about to cite `references/evaluation/*` ·
> an atom's prose and its example disagree · asked whether clippy or `-D
> warnings` reaches a doctest body
> **See also:** 00 (the five prime directives) · 60 (what a test must prove) ·
> 61 (compile-time assertions) · 62 (doctests and harnesses) · 81 (checks that
> cannot be types)

---

Every atom in this corpus is included as a doctest module of the `xtask` crate —
one `#[cfg(doctest)] mod` per file — so `cargo test -p xtask --doc` is what turns
an example into a claim. That is the only mechanical check this corpus has.
Everything below exists because it is what the compiler cannot check for you.

## RS-01-1. Name the wrong implementation, and compile it beside the rule.

**Why.** Nothing compiles a preference, so a doctrine rule has no failure mode
unless its author supplies one. This is CF-1's bar —
`mutation_coverage::every_rule_has_a_mutant` enforces it for conformance rules —
transplanted to prose, where it is easier to violate because nothing fails when
you skip it.

**Do** — the claim, with the assertion that carries it:

```rust
use std::cell::RefCell;

struct Log(RefCell<Vec<u64>>);

impl Log {
    fn append(&self, value: u64) {
        self.0.borrow_mut().push(value);
    }
    fn read(&self) -> Vec<u64> {
        self.0.borrow().clone()
    }
}

// The rule: an appended event is visible to a subsequent read.
fn rule_appends_are_readable(log: &Log) {
    log.append(7);
    assert!(log.read().contains(&7), "an appended event must be readable");
}

rule_appends_are_readable(&Log(RefCell::new(Vec::new())));
```

**Not** — a rule with no nameable violator. This one *is* violated by a store
that acknowledges an append and drops it, so write that store and end the fence
with the assertion that catches it:

```rust
use std::cell::RefCell;

/// One step wrong, not several: a store broken in many ways proves a rule
/// catches *something*, not that it catches *this*.
struct Defect(RefCell<Vec<u64>>);

impl Defect {
    fn append(&self, _value: u64) {}
    fn read(&self) -> Vec<u64> {
        self.0.borrow().clone()
    }
}

let defect = Defect(RefCell::new(Vec::new()));
defect.append(7);
assert!(!defect.read().contains(&7), "the rule above fails this store");
```

**Rejects.** A rule stated as "prefer X to Y" with no failing shape, added
because a reviewer disliked something once. It reads as doctrine, it is loaded by
every future agent on the strength of its `Load when:` line, and there is no
experiment that would ever remove it — so it accumulates, and the corpus's signal
per token falls for everyone.

**Evidence.** `CONTRIBUTING.md:194 (Write the wrong implementation)` ·
`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:433 (impl Defect for InnerJoinTagStore)` ·
[SPECIFICATION CF-1](../../spec/SPECIFICATION.md#61-the-suites-own-proof-obligation) ·
[ADR-0010](../../.kb/decisions/0010-the-suite-must-prove-itself.md)

## RS-01-2. Give every `compile_fail` fence an error code, and a compiling fence in the same atom.

**Why.** rustdoc 1.97.1 parses the `,E####` suffix and then discards it on
stable — it is documented as a nightly facility for the error index, and PS-36
records the same thing measured on a real port doctest, annotated `E0308` against
a diagnostic whose code is `None`. So a `compile_fail` fence proves only that
*something* failed. A fence that has
decayed into "fails because a path was renamed" reports `ok` forever, and the
compiling fence beside it is the positive control that notices when the surface
it is written against has moved.

**Do** — the positive control. It resolves `happenstance_core`'s exported names,
which the `compile_fail` neighbour deliberately does not, so an ADR-0006-shaped
rename fails *this* fence loudly instead of becoming one more reason the
neighbour reports `ok`:

```rust
use happenstance_core::{SequencePosition, StoreId};

fn assert_send<T: Send>() {}

assert_send::<SequencePosition>();
assert_send::<StoreId>();
```

**Not** — a fence marked `E0277` that actually fails with `error[E0425]: cannot
find type `NoSuchTypeExists` in this scope`. rustdoc reports this doctest as
passing, and would keep doing so if the bound it was written to prove were
deleted tomorrow:

```rust,compile_fail,E0277
fn assert_send<T: Send>() {}

assert_send::<NoSuchTypeExists>();
```

**Rejects.** An atom whose `compile_fail` example was written against
`happenstance_runtime::EventStore`, survived ADR-0006's rename untouched, and
now fails with `E0433: failed to resolve` instead of the bound it claims to
demonstrate. The gate is green, the atom looks proved, and the constraint it
documents has been unchecked since phase 0.

**Evidence.** `xtask/src/constitution.rs:22 (cfg(doctest))` ·
`spec/SPECIFICATION.md:6177 (rustdoc on stable 1.97.1)` ·
[rustdoc unstable features](https://doc.rust-lang.org/rustdoc/unstable-features.html#error-numbers-for-compile_fail-doctests)
*(checked 2026-08-09, rustc 1.97.1)*

## RS-01-3. Cite the tree, not the audit: `references/evaluation/*` is dated evidence, never a rule.

**Why.** Those files are pinned to a commit and immutable by design, so their
findings are a measurement of a past tree. A finding may legitimately force a
rule; an unexecuted *recommendation* in one is not a decision, and a defect it
records may already be fixed — §16's was, and VT-13 is where the answer now
lives. Precedence runs SPECIFICATION clause > ADR > atom
> `CLAUDE.md`/`CONTRIBUTING` summary > `references/evaluation/*`, and inside an atom
the compiled example beats the prose.

**Do** — check the claim against the tree as it is today:

```rust
use happenstance_core::SequencePosition;

// §16 of the API-guidelines audit calls this a confirmed defect. It was fixed:
// `next()` is `checked_add`, so the top of the key space reports exhaustion.
let Some(last) = SequencePosition::new(u64::MAX) else {
    return;
};
assert_eq!(last.next(), None);
assert_eq!(SequencePosition::FIRST.next(), SequencePosition::new(2));
```

**Not** — the saturating implementation the audit describes, still shipped
because an atom promoted a dated finding into a standing rule:

```rust
fn next_saturating(position: u64) -> Option<u64> {
    Some(position.saturating_add(1))
}

// The bug the audit found: the one method whose purpose is signalling overflow
// reports it as progress, and a consumer resuming from here re-reads forever.
assert_eq!(next_saturating(u64::MAX), Some(u64::MAX));
```

**Rejects.** An agent handed "fix the confirmed defects in the audit", which
reads §16, does not open `event.rs`, and lands a patch that reverts `checked_add`
to the `saturating_add` the audit's own diff suggests — restoring a defect while
citing the document that reported it, with a commit message that looks
authoritative to review.

**Evidence.** `crates/happenstance-core/src/event.rs:278 (checked_add)` ·
`references/evaluation/research-rust-api-guidelines.md:677 (Confirmed defect)` ·
[SPECIFICATION VT-13](../../spec/SPECIFICATION.md#vt-13--next-signals-overflow-and-the-inclusiveexclusive-asymmetry-is-deliberate) ·
[SPECIFICATION §1.1](../../spec/SPECIFICATION.md#11-what-this-document-is)

## RS-01-4. Assume nothing lints a fence body.

**Why.** `cargo clippy` never invokes rustdoc, so it does not lint doctests at
all; Cargo's `[lints]` tables become flags on the crate's own targets and do not
reach rustdoc's harness; and `RUSTDOCFLAGS=-D warnings` gates the doc *build* —
intra-doc links, `missing_docs` — not anything lexically inside a fence. The
workspace's `unwrap_used = "deny"` is therefore unenforced in every example here,
which is why `lint-constitution` greps the fences for the two call spellings it
forbids — `unwrap` and `expect` — instead of trusting a lint.

**Do** — the shape the house rules ask for, written without an `unwrap` because
nothing would have stopped one:

```rust
use happenstance_core::{Tag, Tags};

let Ok(tags) = Tags::from_pairs([("course", "c1")]) else {
    return;
};
assert_eq!(tags.len(), 1);

let Ok(tag) = Tag::new("course:c1") else {
    return;
};
assert_eq!(tag.as_str(), "course:c1");
```

**Not** — compiles, runs, passes, and is seen by no lint in the gate.
`clippy::needless_range_loop` denies this everywhere else in the workspace, since
CI passes `-D warnings`:

```rust
let bytes = [1u8, 2, 3];
let mut total = 0u32;
for i in 0..bytes.len() {
    total += u32::from(bytes[i]);
}
assert_eq!(total, 6);
```

**Rejects.** An atom stating "no `unwrap` in library code" whose own example
calls `unwrap` — the single most quoted paragraph in the corpus teaching the
opposite of what it says, green under `cargo xtask ci` forever, and copied into
an adapter by the next agent that loads it because a compiled example is exactly
what agents trust most.

**Evidence.** `xtask/src/constitution.rs:27 (Doctests also do not receive the workspace)` ·
`Cargo.toml:233 (unwrap_used = "deny")` ·
[rust-clippy#1599](https://github.com/rust-lang/rust-clippy/issues/1599)
*(checked 2026-08-09, rustc 1.97.1)*
