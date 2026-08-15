# The extension surface, met from outside — the reading discipline and the gap record

This is the primary output of `documented-extension-surface` (HS-S0015). The fixture it describes is
secondary: a fixture proves that *one* author got through, and only this record says **where the
documentation ran out** for them.

Everything above the horizontal rule was written and committed **before** any line of
`examples/outside-projection-adapter/` existed. That ordering is the whole of AC-007's check — a
denylist written afterwards describes what happened; one written first constrains it.

## The arm in force, read and not chosen

DT-8 asks whose adapter-author bar the suite holds. The answer is **not** made here. It is read out
of the signed-off design record, which `projection-api-design-record` (HS-S0014) wrote, and quoted
verbatim:

> ### DT-8 — whose adapter-author bar the suite holds
>
> **Resolution: the bar is held for an author this repository did not write.** The extension surface
> is the documented pair **`projection_store_conformance!` + `ProjectionProbe`**.

— `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md:359-363`.

**Arm A is therefore in force**, and with it the obligation the same file names so that it cannot be
quietly dropped:

> `documented-extension-surface` (HS-S0015) must build a projection fixture **from the documentation
> alone** — not copied from `crates/happenstance-testkit/src/fixtures.rs` — and clear the
> mutant-registry exactness check and the capability-skip rule.

— `_design.md:388-395`.

The stated cost the exercise is measuring, also read rather than assumed: *"one feature flag on a
dependency the adapter already has, and no new edge in the dependency graph"* (`_design.md:365-375`,
restating `spec/SPECIFICATION.md`'s own coherence argument for `ProjectionProbe`'s home).

## The allowlist — rendered documentation only

Read as the rendered rustdoc page an outside author actually meets (`cargo doc --all-features`), and
nothing else, while the store, the probe impl and the fixture are being written:

1. `happenstance-testkit`'s crate page — the `//!` block at `crates/happenstance-testkit/src/lib.rs`.
2. `projection_store_conformance!`'s macro page.
3. `for_each_projection_store_rule!`'s macro page, and the three emitter pages it names.
4. `happenstance_testkit::ProjectionFixture`'s page — the fixture trait, its three capability
   constants and its two provided mechanisms.
5. `happenstance_testkit::Capability` and `happenstance_testkit::RuleOutcome`, plus the four
   `&'static str` reason constants beside them.
6. `happenstance_testkit::projection`'s module page and `happenstance_testkit::projection::rules`'
   module page.
7. `happenstance_core::ProjectionStore`'s page and `happenstance_core::ProjectionProbe`'s page,
   together with `Checkpoint`, `Authority`, `CommitError`, `ResetError` and `ProjectionId`.

Two things are *allowed* that a purist might dispute, and they are named here rather than left to be
noticed. Rustdoc renders a trait's **signatures** as well as its prose, so reading `projection.rs`'s
trait declarations is reading the page. And the doctest on `ProjectionStore` — the `ToyStore`
walkthrough — is **published documentation**, on the port's own page, put there precisely so an
implementer has something to copy; copying from it is using the documented surface, not evading the
denylist. Where it was used, this record says so.

## The denylist — not opened while the fixture is written

1. **`crates/happenstance-testkit/src/fixtures.rs`** — foremost. It carries the owned-handle
   reference `ProjectionFixture` implementation, which is precisely what an author would copy.
2. `crates/happenstance-core/src/projection_memory.rs` — `MemoryProjectionStore`'s source, the
   reference *store*.
3. `crates/happenstance-testkit/tests/projection_conformance*.rs` — the testkit's own harnesses,
   including the buffering conformant variant landed by `buffering-conformant-variant` (HS-S0013).
4. `crates/happenstance-testkit/tests/projection_mutation_coverage.rs` and its `mutants/` directory —
   the wrong stores, which include the checkpoint-only analogue this story must invent for itself.

Rule for both lists: **every point at which the allowlist was insufficient is a gap**, recorded below
with what was missing, which denylisted file or which source read answered it, and the documentation
change that would have prevented it. A run that finds no gaps says so in those words, because a
fixture built with no gaps found is either a triumph or a copy, and only the record tells you which
(`discover.md:89-91`).

---

<!-- Everything below this line was written after the fixture. -->
