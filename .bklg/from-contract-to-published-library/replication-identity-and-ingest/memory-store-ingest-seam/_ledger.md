---
item: HS-S0101
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The one door a foreign identity comes in through

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "GIVEN a peer-runner author holding a MemoryEventStore behind &self and a batch of SequencedEvents that some *other* store authored, WHEN they look for a way to put those events into this store, THEN they find exactly one operation — MemoryEventStore::ingest_at_tail(&self, &[SequencedEvent]) -> IngestedAtTail, inherent, additive, inside the existing #[cfg(feature = \"memory\")] module — and they reach it without implementing a trait, enabling a new feature, or constructing a second store."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/ingest.rs — the #[cfg(feature = \"memory\")] mod memory_store_ingest block (:193-231)"
  verifying_test: "crates/happenstance-core/src/memory.rs mod tests (:440) — ingest_at_tail_is_inherent_and_takes_shared_self; plus the item doctest via `cargo test -p happenstance-core --doc`"

- id: AC-002
  criterion: "GIVEN an event authored by store A carrying A's EventId and A's RecordedAt, WHEN the runner ingests it into store B, THEN reading it back from B yields byte-identical id and recorded_at — so the runner can dedupe across the fleet on identity rather than on hope, and ReMintingIngestDoor (which re-mints EventId::new(self.store_id, position) and leaves every other assertion green) fails this and only this."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/ingest.rs — the #[cfg(feature = \"memory\")] mod memory_store_ingest block (:193-231)"
  verifying_test: "crates/happenstance-core/src/memory.rs mod tests (:440) — ingest_preserves_foreign_identity_and_recorded_at"

- id: AC-003
  criterion: "GIVEN store B whose head is at some position it assigned itself, WHEN the runner ingests a foreign event whose own position field is *below* that head, THEN the event lands at B's fresh tail — above every position B has already assigned — and the incoming position is read by nothing, so the runner's projections never have to reason about an event appearing beneath a checkpoint (OriginPositionInsertingDoor fails here)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/ingest.rs — the #[cfg(feature = \"memory\")] mod memory_store_ingest block (:193-231)"
  verifying_test: "crates/happenstance-core/src/memory.rs mod tests (:440) — ingest_assigns_a_fresh_local_position_above_the_head (positions compared against what the store assigned, never literals — CF-6)"

- id: AC-004
  criterion: "GIVEN a peer that re-delivers — the normal case, not the exceptional one — WHEN the runner ingests the same batch twice, or two tasks ingest overlapping batches concurrently, THEN each EventId is stored at most once, the second delivery is reported as skipped rather than appended or refused, and the decision is taken under the same single write lock as the write, so a runner cannot lose a race it did not know it was in."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/ingest.rs — the #[cfg(feature = \"memory\")] mod memory_store_ingest block (:193-231)"
  verifying_test: "crates/happenstance-core/src/memory.rs mod tests (:440) — ingest_skips_ids_the_store_already_holds, plus the two-thread overlapping-batch test over a shared Arc<MemoryEventStore>"

- id: AC-005
  criterion: "GIVEN a runner that must advance its own resume token after a push, WHEN the call returns, THEN it can name the outcome type through the defining crate's root re-export (happenstance_core::IngestedAtTail) and read appended, skipped and last_local from it without matching a tuple or paying for an allocation it did not ask for — and the type is #[non_exhaustive], so a later field is not a breaking change for them."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs — the crate-root re-export, consumed from crates/happenstance-sync/src/ingest.rs:193-231"
  verifying_test: "crates/happenstance-core/tests/frozen_signatures.rs — the out-of-crate #[non_exhaustive] assertion added beside :236-296; plus the root-re-export test in crates/happenstance-core/src/memory.rs mod tests (:440)"

- id: AC-006
  criterion: "GIVEN SY-1's frozen prohibition on ingest re-checking the writer's asserted append conditions, WHEN a future implementer tries to make the receiver reject an event, THEN they find the signature cannot express it — no AppendCondition parameter of any kind, no Result — and an empty batch is a no-op returning appended: 0, skipped: 0, last_local: None rather than the AppendError::NoEvents that append owes, because a peer with nothing new to send is ordinary traffic."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/ingest.rs — the #[cfg(feature = \"memory\")] mod memory_store_ingest block (:193-231)"
  verifying_test: "crates/happenstance-core/tests/frozen_signatures.rs — the fn-type pin `let _: fn(&MemoryEventStore, &[SequencedEvent]) -> IngestedAtTail = MemoryEventStore::ingest_at_tail;`; plus ingest_at_tail_of_nothing_is_a_no_op in crates/happenstance-core/src/memory.rs mod tests (:440)"

- id: AC-007
  criterion: "GIVEN a runner that already handed clones of its Arc<MemoryEventStore> to a projection task and a query path, WHEN it ingests a batch through its own handle, THEN every handle taken *before* the ingest observes the ingested events afterwards — the store is mutated in place behind &self and never rebuilt, which is precisely what RestoreBasedIngest (snapshot() + restore) silently breaks while preserving identity correctly."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/ingest.rs — the #[cfg(feature = \"memory\")] mod memory_store_ingest block (:193-231)"
  verifying_test: "crates/happenstance-core/src/memory.rs mod tests (:440) — ingest_is_visible_through_a_handle_taken_before_it, shaped after crates/happenstance-testkit/src/suite.rs:265; plus `cargo test -p happenstance-testkit --all-features` green"

- id: AC-008
  criterion: "GIVEN an evaluator or an adapter author who has already implemented EventStore against the published 0.2.0 contract, WHEN this change lands, THEN nothing they wrote needs to change: crates/happenstance-core/src/store.rs is byte-identical, no port gained an ingest-shaped method (SY-8 [FROZEN]), no new Cargo feature appeared, and frozen_signatures.rs's existing append_takes_no_identity still compiles and passes **unedited**."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/store.rs — the unchanged port; asserted from crates/happenstance-core/tests/frozen_signatures.rs:236-296"
  verifying_test: "`git diff --exit-code main -- crates/happenstance-core/src/store.rs crates/happenstance-core/Cargo.toml` plus `cargo test -p happenstance-core --test frozen_signatures --all-features` and `cargo xtask spec-trace`"

- id: AC-009
  criterion: "GIVEN an evaluator deciding in one sitting whether to adopt, WHEN they read the new item, THEN it arrives fully presented rather than bare: rustdoc that names the alternative that lost, a compiled doctest showing a real two-store round trip with the origin id surviving and the local position fresh, a CHANGELOG.md [Unreleased] / ### Added entry, and the **semver class stated out loud** (minor) and traceable to ADR-0026 rather than inferred from the diff."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/memory.rs — the rustdoc + doctest on the new item; CHANGELOG.md [Unreleased] / ### Added (:13-22)"
  verifying_test: "`cargo test -p happenstance-core --doc` and the docs step of `cargo xtask ci --fast` (-D rustdoc::broken_intra_doc_links); the CHANGELOG and rustdoc semver lines cited file:line here and checked against the accepted ADR-0026 atom via `redkiln validate --kb`"

- id: AC-010
  criterion: "GIVEN the peer-runner author whose journey today dead-ends at a todo!() whose message *is* this story's absence (crates/happenstance-sync/src/ingest.rs:219-221), WHEN they open the real composition root, THEN the #[cfg(feature = \"memory\")] mod memory_store_ingest block reaches the new door from live code and the whole block compiles under the memory feature — the seam is mounted in the tree, not merely available in a crate nobody calls."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/ingest.rs — the #[cfg(feature = \"memory\")] mod memory_store_ingest block (:193-231), and its module documentation at :38-82 and :200-205"
  verifying_test: "`cargo test -p happenstance-sync --features memory` (the feature is load-bearing — a plain `cargo test -p happenstance-sync` compiles none of the block, crates/happenstance-sync/Cargo.toml:37-44); plus `cargo xtask affected --base main`"
```
