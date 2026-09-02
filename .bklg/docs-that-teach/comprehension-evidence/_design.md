---
item: HS-P0024
stage: design
created: "2026-08-17T04:40:32.000Z"
updated: "2026-08-17T04:40:32.000Z"
---

# API surface design — Comprehension Evidence

## Items

**This project ships no user-facing surface — no screen and no public API.** It
adds no `pub` item, compiles nothing new, and touches no crate under `crates/`.
This is confirmed by the project's own UX brief, not assumed: `_decomposition.md`,
`## UX brief`, states directly — *"This project builds no screen and no public
API."*

The repository itself has nothing for a screen to render into. `rg -n "design
system|design-system|tokens|primitives" CLAUDE.md README.md` returns nothing;
globs for `*token*.css`, `*theme*.css`, `tailwind*`, `*.tokens.*`, and any
`*/ui/**` shared UI package all return zero files. This is a Rust workspace
(`happenstance-core`, `happenstance`, the adapter crates) with a `docs/` tree and
rustdoc, not an app with rendered screens. `.redkiln/config.yaml` (`## Deterministic
gate commands, and design capture`, the block around line 15–81) confirms
`design.capture` is deliberately undeclared — commented out — so the perceptual-
review pipeline is a skip by design: there is no app to screenshot.

That leaves nothing for `## Signatures`, `## Shape decision`, `## Placement and
re-export`, `## Visibility and stability`, `## What it costs a caller`, `## What
a user meets first`, `## The states the API must express`, or `## The doctest`
to review — there is no `pub` item and no doctest to write them against.

What this project's three artifacts actually are, per `_decomposition.md`, `## UX
brief` — none of them composed UI, all of them markdown or rustdoc a person
reads:

1. **The material the reader walks** — the existing, already-assembled docs
   tree: `docs/README.md`, the rendered rustdoc of `crates/happenstance-core/`
   and `crates/happenstance/`, and `examples/course-subscriptions/`. This project
   does not author that surface; it observes a person using it.
2. **The friction log** — a dated markdown artifact whose reader is a reviewer,
   a sibling-project owner, and eventually HS-P0025.
3. **This file, `_design.md`** — the protocol, fixed before recruitment, and the
   sole durable record of DT-9 (which persona the comprehension session walks),
   because `design.capture`'s absence means there is no capture/perceptual gate
   to record it instead.

The brief is explicit that composition for all three is bounded, not free: *"there
is no CSS layer and no token file, and inventing one is out of scope"*
(`_decomposition.md`, `## The primitive layer to compose from — do not
hand-roll`). Anything these three artifacts need is drawn from repository
*document* primitives that already exist and are verified present in this repo:
`.redkiln/templates/briefs/brief.md`'s Intent/Acceptance-Criteria/Notes spine,
`.redkiln/templates/_design.md`'s own `## Shape decision` table and `## Sign-off`
section, the `| Risk | Likelihood / Impact | Mitigation |` risk-table shape
already used across this initiative's `project.md` files, the two-column routing
table `docs/README.md` already uses, and the one-line-checkbox primitive from
`.redkiln/templates/gates/` (load-bearing: the gate parser matches line by line
and a wrapped box never matches). None of these is a token, a component, or a
CSS class — this is a text-corpus repository, and there is nothing here to name
that has not already been named above.

```yaml
# no items — no public API surface, no rendered UI surface
```

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

No user-facing surface. Nothing here requires human sign-off on a screen, a
token, or an API signature; the only decision this file itself durably records —
DT-9, which persona the comprehension session walks — is sign-off by the human
reading this file, since no capture/perceptual gate exists to record it
elsewhere.

**That sign-off has now happened: approved by Ryan Britton (repository owner),
2026-08-17, with no conditions.** Recorded via
`redkiln advance HS-P0024 --verdict approved --stay --apply`. The verifier found no
gaps on this project. `hasSurface: false` was recorded explicitly rather than the
project being silently skipped, which is what the design stage requires of a project
with nothing to draw.
