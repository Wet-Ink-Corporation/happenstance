---
id: kb-decision-0076
title: The emitters CF-23 obliges are public API — un-hidden, renamed without the prefix, and pinned by CF-41
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0076
reversibility: low
phase: 17
supersedes: null
superseded_by: null
summary: >-
  Executes the policy ADR-0066 set for kb-open-question-cf-23-emitter-names-unstable-001 and
  resolves the conflict that record left between its own sections. ADR-0066 put the emitter names
  CF-23 obliges an adapter to write inside the 1.0 promise, while its section 5 exempted every
  doc(hidden) item from semver, and every emitter was doc(hidden) with a __ prefix. This record
  resolves it by un-hiding rather than by adding an exception. The ten conformance emitters
  (event-store, projection, model and concurrency families) drop the prefix and the attribute and
  become ordinary documented macros: emit_tokio, emit_blocking, emit_wasm, emit_projection_tokio,
  emit_projection_blocking, emit_projection_wasm, emit_model_tokio, emit_model_blocking,
  emit_concurrency_tokio and emit_concurrency_blocking. They render on docs.rs, and
  cargo-semver-checks, which skips hidden items, can now see them. __emit_rule_names becomes
  __rule_names and stays hidden, because it wraps no test. The two benchmark emitters stay hidden
  and unpromised, because a benchmark is not the bar (CF-34). The rule from here is that __ means
  not promised and no promised name carries it. The renames are hard, with no aliases, in the
  0.4.0 breaking window. ADR-0066 section 8 records no reverse dependencies outside this workspace,
  so nobody has to migrate. CF-41 is minted FROZEN, and its rule is a committed-list meta-test
  with two negative controls. This record supersedes ADR-0066 section 5 in part, only where the
  doc(hidden) exemption would have covered a promised emitter. It supersedes no other part, so
  kb-decision-0066 stays accepted. What a hand-written emitter must name inside a suite macro's
  expansion, __conformance_fixture and $crate::__private, is not promised.
depends_on:
  - kb-decision-0066
related:
  - kb-decision-0072
  - kb-decision-0057
  - kb-open-question-cf-23-emitter-names-unstable-001
source_paths:
  - crates/happenstance-testkit/src/registry.rs
  - crates/happenstance-testkit/src/projection.rs
  - crates/happenstance-testkit/src/model.rs
  - crates/happenstance-testkit/src/concurrency.rs
  - crates/happenstance-testkit/src/bench.rs
  - crates/happenstance-testkit/src/lib.rs
  - crates/happenstance-testkit/tests/emitter_surface.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-30
---

# The emitters CF-23 obliges are public API — un-hidden, renamed without the prefix, and pinned by CF-41

## The question

CF-23 `[FROZEN]` makes the per-test wrapper a parameter the adapter supplies. The testkit's emitter
macros are the only concrete values of that parameter, and `happenstance-cloudflare`, which has no
`#[tokio::test]`, can reach the suite only by naming one. Until `0.4.0` every emitter was declared
`#[doc(hidden)] #[macro_export] macro_rules! __emit_…`.

ADR-0066 decided the policy: the names CF-23 obliges are **inside** the 1.0 promise. Its §5 also
exempts every `#[doc(hidden)]` item from semver. Read together, the two sections promise a set of
names and exempt every name in that set. The open question left the renames to phase 17, and this
record is where they are decided.

## Decision

**Un-hide the promised names; do not add an exception for them.**

1. **The ten conformance emitters become ordinary public macros.** The `__` prefix and
   `#[doc(hidden)]` are both dropped, and the `family_runtime` structure stays, with the
   event-store family unprefixed as it was:

   | Was | Is |
   |---|---|
   | `__emit_tokio`, `__emit_blocking`, `__emit_wasm` | `emit_tokio`, `emit_blocking`, `emit_wasm` |
   | `__emit_projection_{tokio,blocking,wasm}` | `emit_projection_{tokio,blocking,wasm}` |
   | `__emit_model_{tokio,blocking}` | `emit_model_{tokio,blocking}` |
   | `__emit_concurrency_{tokio,blocking}` | `emit_concurrency_{tokio,blocking}` |

   Each now carries rustdoc that renders on docs.rs, names the suite macro it is passed to, and
   states the promise.
2. **`__emit_rule_names` becomes `__rule_names`**, hidden and unpromised. It is not an emitter: it
   expands a rule enumeration to a `[&str; N]` for the testkit's own meta-tests, and naming it
   `emit` suggested an obligation it never carried.
3. **`__emit_benchmark_tokio` and `__emit_benchmark_blocking` keep their names and stay hidden.**
   CF-23 obliges a *conformance* wrapper, and CF-34 says benchmarks are not the bar. Each now
   carries a doc line saying it is outside the promise.
4. **The rule is `__` means not promised.** An exported macro whose name starts with `__` carries
   `#[doc(hidden)]`; a hidden one starts with `__`; no promised name carries the prefix. CF-41
   states it and a test holds it.
5. **Hard rename, no aliases.** An alias would be a second `macro_rules!` forwarding to the first,
   and it would need a `#[doc(hidden)]` of its own to stay off the page. That is the shape this
   record exists to retire, kept for a population that does not exist. ADR-0066 §8 records no
   reverse dependency outside this workspace. The break is paid once, in `0.4.0`, alongside the
   window's other breaks (ADR-0072).
6. **The hand-written emitter's handshake is not promised.** An emitter expands inside the suite
   macro's generated module and names two things the suite defines: a `__conformance_fixture`
   function, and each family's rules under `$crate::__private`. Promising those would freeze the
   suite's internals for the one caller who writes their own emitter. The front page instead tells
   that caller to start from a shipped emitter's body and to follow this crate's changelog.

### Why un-hiding and not an exception (Rust background)

`#[doc(hidden)]` does not restrict access. It only removes an item from rendered documentation.
`macro_rules!` macros exported with `#[macro_export]` all live at the crate root, in one flat
namespace, and cannot be placed behind a private module. So a `macro_rules!` helper a downstream
expansion must reach has to be exported, and `#[doc(hidden)]` plus a `__` prefix is the
community's way of saying "exported because the language forces it, not because it is a promise".
`cargo-semver-checks` honours that convention and skips hidden items.

The alternative was to keep the `__emit_*` spellings and add a named exception to ADR-0066 §5. It
lost for three reasons:

- **The names would still say the opposite of the promise.** Anyone who knows the convention
  would read a `__` name as unstable, whatever a clause said.
- **The only instrument would be a hand-kept list.** `cargo-semver-checks` would stay blind, and
  CF-41's pinned test would be the whole of the promise.
- **The exception would be permanent.** A 1.0 charter carrying an exception for a spelling chosen
  before the promise existed is a debt that never gets paid down.

Un-hiding costs one rename per name, in a release that already breaks.

## CF-41

Minted `[FROZEN]` in §6.6. The rule is
`crates/happenstance-testkit/tests/emitter_surface.rs`'s
`the_promised_emitters_are_exactly_the_pinned_list`, which compares the exported emitters against a
**committed** list rather than a derived one. A rename edits the definition and every in-tree caller
together, so no compile and no derived comparison can see it; only a list written down separately
can. The wrong implementations it names are the `0.3.2` shape and the one-edit rename. Two
negative controls feed it those shapes and assert refusal:
`the_rule_refuses_the_hidden_names_that_shipped_before_it` and
`the_rule_refuses_a_hidden_promise_and_a_rendered_exemption`.

It is frozen now rather than provisional because it has no falsifier to wait on. The clause is
a naming policy held by a test, and the test fails on its named wrong implementation.

## What this supersedes, and what it does not

**ADR-0066 §5, in part:** the `#[doc(hidden)]` exemption no longer reaches a name CF-23 obliges,
because no such name is hidden any more. The exemption otherwise stands: `__rule_names`, the
benchmark pair, `__private`, and every other hidden item remain outside semver. Nothing else in
ADR-0066 changes. `kb-decision-0066` therefore keeps `status: accepted`, as `kb-decision-0022` did
under ADR-0065 and ADR-0068. The decision map annotates its row instead.

## Consequences

- **`0.4.0`'s semver trace needs a hand-written row for this record.** The removed names were
  hidden, so `cargo-semver-checks` against `0.3.2` cannot report their removal. From `0.4.0` on
  it can see the new names. Whether its macro-removal lint fires on a declarative macro was not
  measured here. CF-41's test does not depend on it.
- **In-tree callers moved in the same change:** the conformance targets of `happenstance-sqlite`,
  `-postgres`, `-neon`, `-cloudflare` and `happenstance-ladybug` (unpublished, built and run
  locally once), the testkit's own targets, `xtask`'s wasm registry, the specification's prose,
  and `standards/rust/41` and `52`.
- **`kb-open-question-cf-23-emitter-names-unstable-001` closes.** Its three sub-questions are
  answered by items 1 and 4, CF-41's pinned test, and item 2.
