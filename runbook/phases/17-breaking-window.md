# Phase 17 — The breaking window, released as `0.4.0`

**Goal.** Every change to a published crate that 1.0 needs and that breaks a
signature, decided and landed in one release.

**Why here.** Under `0.x`, `cargo-semver-checks` reports a break and the minor
version absorbs it; after `1.0.0` the same break costs a major and every
downstream manifest. Several decisions the roadmap would otherwise make later —
ADR-0028's and half of ADR-0026's — have an answer that adds or changes a method
on a published trait, so they are made here, before the phases that implement
them. One release rather than several, because each breaking minor asks every
adopter to move once.

**Decisions it settles.** ADR-0028 (moved from phase 14). The published-surface
half of ADR-0026. The `Projection::apply` record the port's freeze left owed.
Whatever phase 16 classified as breaking.

**Work**

- [ ] **ADR-0028 — what a store may forget, and how it says so.** Either a port
      surface through which a store reports history it no longer holds (a method on
      `EventStore`, which breaks all four published adapters) or a written refusal
      stating what a store that has been deleted from may look like. Phase 14 then
      builds the suffix store and the rules against whichever this is.
- [ ] **ADR-0026's published half — a foreign identity's write path.**
      `crates/happenstance-sync/src/ingest.rs` holds four `todo!()` bodies
      blocked on `happenstance-core` having no write path that preserves a foreign
      `EventId` (VT-10). Decide whether core grows one, or each adapter does, or
      `IngestStore` works without one — before 1.0, because the first two touch
      published crates. Settle it against a compiling spike, not an argument.
- [ ] **`Projection::apply`** — synchronous, asynchronous, or given a batch
      handle it can issue statements through
      (`.kb/open-questions/projection-apply-is-synchronous-against-a-live-store.md`).
      Phase 18 implements it.
- [ ] **The breaking open questions phase 16 listed**, each answered in its own
      record or closed with a reason. At the split the candidates were codec
      sealing, a `Busy` variant on `AppendError`, the projection-batch statement
      type, `ProjectionId` validation, the `trait-variant` caret, a feature table
      for `happenstance-cloudflare`, the empty-emission idiom, heterogeneous tuple
      boundaries, the read-page budget and ES-17's append ownership.
- [ ] **Whether `happenstance-core`'s empty `unstable-projection` feature goes.**
      It gates nothing and is kept so `0.2.0` manifests resolve; removing a feature
      is a break, so if it goes, it goes here.
- [ ] **Release `0.4.0`.** `cargo-semver-checks` against the `0.3.x` registry
      baseline reports breaks, and each one it reports traces to a decision above.
      `CHANGELOG.md`'s `[Unreleased]` entries — SQLite's fifteen-second busy
      timeout (ADR-0065) and `FaultyStore::contend_next` — ship in `0.4.0`;
      there is no `0.3.3` (`wi-052920`).

**Proof artefact.** `0.4.0` on crates.io, and a table in its changelog entry
mapping every major finding `cargo-semver-checks` reported to the decision that
caused it — a break with no row is one nobody decided.

**Exit criteria**

- [ ] ADR-0028 accepted; ES-39 is no longer `[DEFERRED]` on a surface 1.0
      promises, or is renewed past 1.0 with the disposition phase 16 gave it.
- [ ] The foreign-identity question is answered against a compiling spike.
- [ ] The `apply` record is accepted.
- [ ] Every open question phase 16 classified as breaking is answered or closed.
- [ ] `0.4.0` is released and its semver findings are fully traced.

**Estimate.** 5–8 days. The range is honest: the spread is mostly the
foreign-identity spike, which nobody has attempted.

**Session log**
