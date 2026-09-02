---
item: "HS-P0013"
stage: design
created: "2026-08-11"
updated: "2026-08-11"
---

# API surface design — The edge store, run rather than asserted

## Surfaces

**N/A — no user-facing surface.** This project ships no screen, CLI TUI, or
documentation site. It turns `crates/happenstance-cloudflare` from a `todo!()`
instrument into a real `EventStore` adapter against Cloudflare Durable
Objects' `SqlStorage` API, gives it a `Fixture`, and wires an off-tokio
(`workerd`/`wasm32`) conformance run into `cargo xtask ci`. What it touches
instead of a screen is a Durable Object's SQL storage, an `xtask` step, and a
decision record (ADR-0023).

Verified directly rather than assumed, by reading this project's own scope
rather than inferring it:

- `project.md`'s **In scope** list is entirely: real `SqlStorage` bindings,
  a `Fixture` for the Durable Object, the off-tokio conformance harness wired
  into `xtask`, ADR-0023, the ES-6 error-type verdict, the two capability
  limits (cursor snapshot, 2^53 position ceiling), CF-39/CF-40, CF-14/CF-27
  re-reads, the ES-32 tail-seam paragraph, WF-11's falsifier, and the
  crates.io name reservation. None of these renders anything
  (`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md`,
  **In scope**).
- The project's own companion briefs file states plainly: "`ux` is not
  warranted for this project"
  (`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md:5-6`,
  citing `.bklg/from-contract-to-published-library/_decomposition.md:258`).
  That row's Warranted-briefs table has architecture, testing and deployment
  checked and the `ux` column blank — the same table `sqlite-durable-store`
  and `postgres-and-neon-stores` share, none of which produces a UI.
- The initiative itself carries `userFacing: false`, and `initiative.md`
  records that no `interaction-patterns.md` was produced "deliberately, at
  the intake gate" — verified missing by `test -f` against
  `.bklg/from-contract-to-published-library/_discovery/distillation/interaction-patterns.md`.
- This project's **Out of scope** list
  (`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md`,
  **Out of scope**) and its non-goals in `_decomposition.md:112-145` name only
  backend/process items — the capability-reporting policy, the tail seam, and
  publishing — none of them UX-shaped.
- Its three warranted briefs (architecture, testing, deployment;
  `_decomposition.md:13-866`) describe Rust modules, SQL rendering, `Fixture`
  capability constants, `xtask` step wiring and CI runner matrices — zero
  mentions of a route, a view, a DOM node or a rendered control.

Sanity check performed directly against this run: grepping the project's
scope for anything that renders turns up nothing — the only "surfaces" named
anywhere in its briefs are Rust module boundaries (Architecture brief §4,
"Composition roots — where each capability MOUNTS") and those mounts are a
constructor (`CloudflareEventStore::new`), a Durable Object host class, a
test target, and a `Fixture` impl — code, not screens.

```yaml
```

## Items

N/A — no user-facing surface.

## Signatures

N/A — no user-facing surface.

## Shape decision

N/A — no user-facing surface.

## Placement and re-export

N/A — no user-facing surface.

## Visibility and stability

N/A — no user-facing surface.

## What it costs a caller

N/A — no user-facing surface.

## What a user meets first

N/A — no user-facing surface.

## The states the API must express

N/A — no user-facing surface.

## Anti-patterns

N/A — no user-facing surface.

## The doctest

N/A — no user-facing surface.

## Sign-off

N/A — no user-facing surface.

**Approved.** Ryan Britton (repository owner), 2026-08-12, at the `/redkiln:plan` design
sign-off gate. No mock was produced or owed: this project records no user-facing surface, and
that is stated explicitly here rather than left as a silent skip. What was approved is the
no-surface determination itself together with the anti-patterns recorded above — the design
stage's `design.capture` perceptual review remains a **declared** skip, per `CLAUDE.md`.
Conditions: none.
