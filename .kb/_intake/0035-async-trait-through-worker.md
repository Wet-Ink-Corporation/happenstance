# ADR-0035 — `async-trait` is exempted where it is reached through `worker`

Staged for a `/redkiln:kb-ingest` wave. The long-form record is
`references/adr/0035-async-trait-through-worker.md`; the atom this produces must
link it rather than restate it.

## What was decided

`deny.toml`'s `async-trait` ban gains two wrappers — `worker` and
`worker-macros` — ratified by ADR-0035, which **amends ADR-0001's exemption set**
without touching ADR-0001's body or reasoning.

The maintainer made this call explicitly at HS-P0013's gate on 2026-08-20, after
both shapes had been named and neither chosen by two earlier passes: HS-S0059
refused it (outside its PR boundary, and the identical edit had been reverted as
finding-deletion at `2ea99fd`), and the phase-9 ingest wave refused it (ratifying
an exemption to a binding constraint against a red gate with no human present is
what `.kb/decisions/README.md:31-33` forbids).

## The claim cluster

1. **The ban is a proxy, not an end.** ADR-0001 forbids `#[async_trait]` because
   it injects `+ Send` into every method it desugars, which forecloses the
   `wasm32` / Workers target where the store is `!Send` by construction.

2. **`worker` does not violate that property.** It uses `async-trait` for its own
   traits; this workspace neither implements them nor derives ports from them, so
   no `happenstance` port passes through the macro.

3. **The load-bearing guard is not `cargo deny`.**
   `crates/happenstance/tests/flavours.rs` instantiates every typed-layer entry
   point against a genuinely `!Send` store and stops compiling the moment a `Send`
   bound reaches the chain. `cargo deny` is an optional probed gate step;
   `flavours.rs` is a compile-time obligation on every target. The exemption
   removes the legible guard on two edges and leaves the unskippable one on all.

4. **The two new wrappers rest on a DIFFERENT argument from the existing one, and
   the atom must not collapse them.** `wasm-bindgen-test` is exempt because it is
   a dev-dependency appearing in no published artifact. `worker` is a *normal*
   dependency of `happenstance-cloudflare` and does ship. Reusing the first
   sentence for the second case would be reasoning about a different situation
   with the same words.

5. **Exempting by name is what keeps the ban real.** A third route into the graph
   still fails the gate until someone decides it should not.

## What this resolves

`kb-open-question-worker-async-trait-ban-001`
(`.kb/open-questions/deny-bans-red-on-the-worker-dependency.md`), minted by wave
`2026-08-20-intake-phase-9` precisely because neither shape had been chosen. Its
`status` flips and it gains `related`; **its body is not reworded** — what it
describes was true when written.

## Two things the wave must NOT do

1. **Do not amend or merge into `.kb/decisions/0001-async-port-flavours.md`.** It
   is accepted and therefore immutable. ADR-0035 amends its *exemption set* from
   outside, the way ADR-0029 amended ADR-0004's number without touching its body.
   The only legal shapes for decision content are a new atom or a superseding one.

2. **Do not read this as resolving the three store-limit numbers.** HS-P0013's
   run-2 review found them read off a platform page while HS-S0055 AC-001 says in
   terms *"never read off a platform page"*. No document has adjudicated that,
   ADR-0023 did not, and ADR-0035 does not. It is still owed, and it surfaces in
   HS-S0055's own record rather than here.

## Known gap, recorded rather than hidden

`worker 0.8.5` is pinned by what it drags. A major bump changing how `worker`
uses `async-trait` would not be caught, because the wrapper *name* is unchanged.
This is the same gap the `wasm-bindgen-test` entry has carried since the ban's
first run, and it is a property of `wrappers` rather than of this decision.

## Evidence

The failure this removes, before the change:

```text
error[banned]: crate 'async-trait = 0.1.91' is explicitly banned
  ├ async-trait v0.1.91
    ├── wasm-bindgen-test v0.3.76        ← already exempted
    ├── worker v0.8.5
    │   └── happenstance-cloudflare v0.2.0-alpha.1
    └── worker-macros v0.8.5
        └── worker v0.8.5 (*)
```

After: `cargo deny check bans` reports `bans ok`.

Consequence for the backlog: HS-S0059's AC-012 becomes claimable as written —
`cargo xtask ci` green at every step — rather than needing its acceptance
sentence reworded to match a red one.
