---
id: "kb-decision-wi-690259"
title: "Adapter owns a documented re-mint"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Should happenstance-cloudflare own the identity re-mint after a Durable Object point-in-time restore, as a documented re-mint operation mirroring SqliteEventStore::remint_identity?\", facing VT-6 lets a mint-once adapter stand only with restore detection or a documented re-mint, and happenstance-cloudflare has neither, we decided for Adapter owns a documented re-mint and neglected Mint fresh on every open; Belongs outside happenstance, on the premise that deployments can be relied on to call it after a bookmark restore, accepting that if wrong: an operator forgets and identities collide silently."
depends_on: []
related:
  - kb-decision-0014
  - kb-decision-0086
source_paths:
  - crates/happenstance-cloudflare/src/event_store.rs
  - crates/happenstance-cloudflare/src/host.rs
  - crates/happenstance-cloudflare/src/lib.rs
  - crates/happenstance-cloudflare/README.md
  - references/adapter-shapes.md
last_reviewed: "2026-10-11"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-690259"
question: "Should happenstance-cloudflare own the identity re-mint after a Durable Object point-in-time restore, as a documented re-mint operation mirroring SqliteEventStore::remint_identity?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-11T17:28:46Z"
tree_hash: "1cd4315d9e55aea6044402e9c159e7b393b256cb"
recommended: "A"
flip_condition: "a restore signal makes it detection instead of a manual call"
---

# Adapter owns a documented re-mint

## Context and problem statement

Should happenstance-cloudflare own the identity re-mint after a Durable Object point-in-time restore, as a documented re-mint operation mirroring SqliteEventStore::remint_identity?

VT-6 lets a mint-once adapter stand only with restore detection or a documented re-mint, and happenstance-cloudflare has neither.

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-690259`. Anchor: `crates/happenstance-cloudflare/src/event_store.rs:63`.

## Decision drivers

- VT-6 conformance
- Public API

## Considered options

### A · Adapter owns a documented re-mint (chosen)

Add a remint operation on CloudflareEventStore, record it in adapter-shapes §7, test a simulated restore; prefer detection if the platform exposes one

- Holds if deployments can be relied on to call it after a bookmark restore.
- If wrong: an operator forgets and identities collide silently.

### B · Mint fresh on every open

Comply with VT-6 by minting per wake, no API

- Holds if watermark growth from routine evictions is tolerable.
- If wrong: every peer's watermark gains a row per eviction, forever.

### C · Belongs outside happenstance

Letterpress owns re-mint; adapter records the gap

- Holds if no other Cloudflare consumer exists.
- If wrong: the adapter stays non-conformant to VT-6 for every user.

## Evidence

- `crates/happenstance-cloudflare/src/event_store.rs:63`: store_meta carries this object's own StoreId, minted once at the first migrate
- `spec/SPECIFICATION.md:894`: An adapter that can do neither MUST mint a fresh incarnation on every open
- `spec/SPECIFICATION.md:856`: PROVISIONAL, falsified by ... a Cloudflare Durable Object, whose isolate is evicted and revived as a matter of routine
- `crates/happenstance-sqlite/src/event_store.rs:670`: pub fn remint_identity(path) -> Result<StoreId, _>

## Decision outcome

Chosen option: **Adapter owns a documented re-mint**, the recommended option.

Decider's note: Ryan, 2026-10-11 in thread: A

### Consequences

- Good, because it holds if deployments can be relied on to call it after a bookmark restore.
- Bad, because if wrong: an operator forgets and identities collide silently.

### Confirmation

Revisit when: a restore signal makes it detection instead of a manual call

## Why this might be wrong

if restores happen through tooling that cannot call the adapter

## Provenance

- Decided 2026-10-11T17:28:46Z by human:ryan (user-approved), via chat.
- Raised in session `3f575cc9-ab0b-5697-b42d-65d591f5b793`.
- Verbatim: "Should happenstance-cloudflare own the identity re-mint after a Durable Object point-in-time restore, as a documented re-mint operation mirroring SqliteEventStore::remint_identity?"
