---
item: HS-S0052
stage: implement
created: 2026-08-12T13:46:50.413Z
updated: 2026-08-12T13:46:50.413Z
---

# Acceptance ledger — The ES-6 artefact: constraint violation versus transport fault, recovered by a caller

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two rows carry a conditional shape settled at implementation, not here. AC-006's `verifying_test`
names the host-native tier (the testing brief's recommended default); if `cargo test -p
happenstance-cloudflare` stops linking once `worker` is real, the same tests move behind
`#[cfg(all(test, target_arch = "wasm32"))]` and re-emit through `wasm_bindgen_test` — the evidence
cites whichever ran, and the probes move with them rather than being deleted (spec EC-004). AC-003's
evidence must additionally cite the accessor added under EC-003, if one was needed.

```yaml
- id: AC-001
  criterion: "GIVEN a library consumer appending under an AppendCondition against a Durable Object whose SQLite refuses the write with the real text `UNIQUE constraint failed: event.position`, WHEN they call append on a store built the only way there is — CloudflareEventStore::new(sql) — THEN they receive Err(AppendError::ConditionViolated) and nothing else, so their next move is \"re-read and rebuild the decision model\" and never \"retry the transport\". The thrown value is constructed in-test with that exact text, not round-tripped through a live Durable Object, so the artefact stands whether or not the workerd runner exists yet"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs"
  verifying_test: "crates/happenstance-cloudflare/src/lib.rs::tests::constraint_violation_reaches_the_caller_as_condition_violated (cargo test -p happenstance-cloudflare)"
- id: AC-002
  criterion: "GIVEN the same consumer, and a Durable Object that fails for a reason that is not a constraint violation, WHEN append returns, THEN they receive Err(AppendError::Store(CloudflareEventStoreError::Sql(SqlError::Thrown(..)))) and can recover, from the public surface alone, that this was a transport fault and not a conflict — enough to retry rather than rebuild. \"Enough\" is asserted on the variant plus the thrown text reachable through Display or core::error::Error::source, never on which SQL statement ran"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs"
  verifying_test: "crates/happenstance-cloudflare/src/lib.rs::tests::transport_fault_reaches_the_caller_distinguishably (cargo test -p happenstance-cloudflare)"
- id: AC-003
  criterion: "GIVEN a downstream consumer — someone who has only happenstance-cloudflare as a dependency and cannot see its internals — WHEN they attempt the reconstruction AC-001 and AC-002 assert, THEN they can do it: the tests reach the store through CloudflareEventStore::new(sql), bind EventStore (not SendEventStore, CLAUDE.md constraint 4), and read only pub items — no private field, no pub(crate) helper, no #[cfg(test)] back door. If the real worker bindings leave the fact unreachable, the minimal accessor that restores it is added, documented per standards/rust/70-rustdoc-obligations.md, keeps CloudflareEventStoreError #[non_exhaustive], and is recorded in this spec's behavior table on merge"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs"
  verifying_test: "crates/happenstance-cloudflare/src/lib.rs::tests (AC-001 + AC-002 bodies reaching only public paths) plus `cargo doc -p happenstance-cloudflare` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`"
- id: AC-004
  criterion: "GIVEN the adapter author who wants to trust this artefact rather than admire it, WHEN they read the diff, THEN they find a named wrong error shape in this crate's own test tree — D3's classifier that picks the right AppendError arm and then discards the evidence — and the same assertion AC-002 uses rejects it. Modelled on the_probe_is_not_vacuous, which exists because the !Send module would otherwise pass with a broken probe; without it this is a rule no adapter can fail (CLAUDE.md, a rule that no adapter can fail is decorative)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs"
  verifying_test: "crates/happenstance-cloudflare/src/lib.rs::tests::an_evidence_discarding_classifier_is_rejected (cargo test -p happenstance-cloudflare)"
- id: AC-005
  criterion: "GIVEN the gate reader who opens the crate documentation to find out what ES-6 resolved to, WHEN they read the Findings section, THEN it reads as observation against a real worker::Error rather than prediction against a stand-in — findings 2 and 3 in particular — and names adr-0023-and-atom-resolutions as where the atom lands. AND the diff writes no file under .kb/: the verdict is recorded here and minted there, through /redkiln:kb-ingest, never hand-written (project.md, DR-7)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs"
  verifying_test: "crates/happenstance-cloudflare/src/lib.rs:12-21 and :63-95 reviewed in the diff; `git diff --name-only` against the merge base contains no .kb/ path; `redkiln validate --kb && redkiln doctor` clean; docs step of `cargo xtask ci`"
- id: AC-006
  criterion: "GIVEN a contributor with no workerd toolchain — an ordinary inner loop — WHEN they run one cargo test, THEN they reach AC-001 through AC-004 and the four !Send probes: ES-6's information half and its auto-trait half answer to the same command. Which target hosts them is settled at implementation, not before — host-native beside the probes if `cargo test -p happenstance-cloudflare` still links once worker is real (the testing brief's recommended default), otherwise behind #[cfg(all(test, target_arch = \"wasm32\"))] re-emitting through wasm_bindgen_test, with the probes moved too and never deleted"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs"
  verifying_test: "cargo test -p happenstance-cloudflare lists the four new tests and the_probe_is_not_vacuous (or the crate's wasm-bindgen-test invocation if EC-004 fires); `cargo xtask affected --base main` at story close and `cargo xtask ci --fast` at slice close"
- id: AC-007
  criterion: "GIVEN the gate reader again, WHEN the slice merges, THEN nothing this story did weakened a standing detector: the_error_type_is_not_send, the_js_boundary_types_are_not_send, the_send_flavour_does_not_imply_a_send_error and the_probe_is_not_vacuous all still pass; send_shape::send_flavour::SendStoreWithLocalError still compiles; Error's bound is untouched and .kb/decisions/0009-error-send-sync.md unedited; ES-6's [FROZEN] clause is discharged with evidence, not amended"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs"
  verifying_test: "crates/happenstance-cloudflare/src/lib.rs:180-259 green under cargo test -p happenstance-cloudflare; `cargo xtask spec-trace` green with spec/SPECIFICATION.md:2629 unchanged; `redkiln validate --kb` clean"
```
