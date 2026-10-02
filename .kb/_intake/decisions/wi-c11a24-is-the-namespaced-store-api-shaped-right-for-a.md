---
id: "kb-decision-wi-c11a24"
title: "Keep as built"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Is the namespaced-store API shaped right for a 1.0 promise: CloudflareEventStore::namespaced(sql, &TableNamespace), names {ns}_event/_event_tag/_store_meta/_event_type_idx, alphabet [a-z][a-z0-9_]*, max 32 bytes, no 'sqlite' prefix?\", facing it ships in 0.4.0 and becomes a 1.0 promise; after publish, any change is breaking, we decided for Keep as built and neglected Rename or reshape before merge, on the premise that a narrow validated prefix is what tenants need, accepting that if wrong: a wider alphabet later is additive; narrowing is not."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-02"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-c11a24"
question: "Is the namespaced-store API shaped right for a 1.0 promise: CloudflareEventStore::namespaced(sql, &TableNamespace), names {ns}_event/_event_tag/_store_meta/_event_type_idx, alphabet [a-z][a-z0-9_]*, max 32 bytes, no 'sqlite' prefix?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-02T03:52:13Z"
tree_hash: "59bb183de7c9e114e7045b1dcf7e4d1a18520e23"
recommended: "A"
flip_condition: "owner wants another name"
---

# Keep as built

## Context and problem statement

Is the namespaced-store API shaped right for a 1.0 promise: CloudflareEventStore::namespaced(sql, &TableNamespace), names {ns}_event/_event_tag/_store_meta/_event_type_idx, alphabet [a-z][a-z0-9_]*, max 32 bytes, no 'sqlite' prefix?

It ships in 0.4.0 and becomes a 1.0 promise; after publish, any change is breaking.

Raised by an agent (sweep) as a call and captured by Weigh-In as `wi-c11a24`. Anchor: `crates/happenstance-cloudflare/src/namespace.rs:1`.

## Decision drivers

- reversibility

## Considered options

### A · Keep as built (chosen)

namespaced(sql,&TableNamespace), {ns}_table names

- Holds if a narrow validated prefix is what tenants need.
- If wrong: a wider alphabet later is additive; narrowing is not.

### B · Rename or reshape before merge

owner names the change in notes

- Holds if the spelling reads wrong to the owner.
- If wrong: churn for no gain.

## Evidence

- `crates/happenstance-cloudflare/src/namespace.rs:1`: TableNamespace: [a-z][a-z0-9_]*, <=32 bytes, not 'sqlite…'

## Decision outcome

Chosen option: **Keep as built**, the recommended option.

Decider's note: Owner: proceed with the recommendation. Keep the validated design and the {ns}_ table naming; raise TableNamespace::MAX_LEN 32 -> 64 so a mapped UUID tenant key (37 bytes) fits.

### Consequences

- Good, because it holds if a narrow validated prefix is what tenants need.
- Bad, because if wrong: a wider alphabet later is additive; narrowing is not.

### Confirmation

Revisit when: owner wants another name

## Why this might be wrong

tenant keys with uppercase or hyphens need mapping by the caller

## Provenance

- Decided 2026-10-02T03:52:13Z by human:ryan (user-approved), via chat.
- Raised in session `a2f7a327-66cd-4d89-9cfc-eed8abed1ccf`.
- Verbatim: "Is the namespaced-store API shaped right for a 1.0 promise: CloudflareEventStore::namespaced(sql, &TableNamespace), names {ns}_event/_event_tag/_store_meta/_event_type_idx, alphabet [a-z][a-z0-9_]*, max 32 bytes, no 'sqlite' prefix?"
