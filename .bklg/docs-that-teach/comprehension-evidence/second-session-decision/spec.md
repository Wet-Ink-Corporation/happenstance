---
item: HS-S0170
stage: spec
created: 2026-08-17T13:16:22.711Z
updated: 2026-08-17T13:16:22.711Z
template_sig: 87bbf1d0
rendered_sig: aec0ae3f
---

# Spec — Answer the second-session question explicitly

## Context pack

The MUST-READ distilled core (RFC §6.8/D7): the smallest high-signal set of load-bearing DECISIONS this story
must honor, internalized before any code — the ADR trade-offs, the mount/integration points, the persona-journey
slice it realizes — stated as decisions, not a reading list. This layer is self-sufficient on first load; the
deeper artifacts stay behind the signposted, AC-bound anchors below (progressive disclosure, not duplication).

## Behavior

The observable behavior this work must exhibit.

## Integration contract

How this story mounts into the real app — not an isolated component. State:

- **Archetype**: `capability` (a user-observable slice through every layer) or `foundation` (real
  in-tree substrate consumed by a capability slice in this initiative — never a double or a fixme).
- **Slice / milestone**: the milestone id this story is delivered with (its slice-mates).
- **Mount point**: the real composition-root / render-path file this capability wires into (never
  constructed-but-unmounted or reachable only through a test).
- **Wires into**: the real sibling contracts it consumes — the ports, the value types, the testkit
  fixtures — named by path.
- **Public items**: the `path` ids from the project's `_design.md` `## Items` block this story
  implements or changes. Name them. An item no story claims is an item nobody built, and the design
  sign-off becomes a document about work that did not happen.
- **Conformance rule(s)**: the rule id(s) in `suite.rs` that observe this story's behaviour, or the
  explicit statement that it is not adapter-observable and why. A story that changes a port and names
  no rule is a port change nothing can fail.
- **Clause(s)**: the `SPECIFICATION.md` clause ids this story discharges or amends. Changing a
  `[FROZEN]` clause takes a new ADR, not an edit — say which ADR if so.
- **Advances DoD scenario**: which initiative Definition-of-Done scenario this story moves toward green.

## Acceptance Criteria

Framed from USER INTENT — a persona goal crossing the full stack (GIVEN a <persona> <context>, WHEN they
<action>, THEN <observable, humane outcome>), not a bare capability ("rename works"). Each maps to a real-path
test.

- AC-001:

## Surface quality

The invariants this story must honor as first-class, blocking criteria — never demoted to a
non-functional footnote.

**This section replaces the bundled template's interaction-quality list**, which is about screens. A
Rust library has the same hole in a different medium, and the substitution is not a softening: every
bullet below names something that a green conformance suite, a green clippy and a green `missing_docs`
are all satisfied by. What it is not allowed to become is a list of invariants no story can fail —
strike a bullet that cannot fail here rather than ticking it.

**Write each one as an `AC-###` row in the table above, not as a bullet here.** `redkiln verify`
extracts acceptance criteria by matching `| AC-001 |` and `- AC-001:`; prose bullets in this section
match neither, so an invariant left as a bullet gets no ledger row, is never gated, and is never
tested. List the ids that carry each invariant below and how each is verified.

**Surface invariants** — is the thing a caller meets the thing that was designed? Take these from the
project's `_design.md`, not from first principles; this story implements that surface, it does not
re-decide it:

- **The surface exists as designed** — signatures match `## Signatures`, item for item. A body that is
  correct behind a signature nobody agreed to is a different deliverable.
- **Both flavours** — anything touching a port type-checks on the bare *and* the `Send` flavour, or the
  spec says which one it deliberately does not serve and why (ADR-0001).
- **The example compiles** — the doctest from `_design.md`'s `## The doctest` exists in the source and
  is run by the gate. A described example is not a checked one.
- **Errors are documented** — every fallible public function carries an `# Errors` section naming the
  conditions, not the error type.
- **Visibility is as signed off** — `pub` / `pub(crate)` / `#[non_exhaustive]` / feature gate exactly as
  `## Visibility and stability` states. An item that became `pub` during implementation is a semver
  promise nobody made.
- **Anti-patterns** — none of the forbidden moves named in `_design.md` or in `CLAUDE.md`'s binding constraints appear.

**Suite invariants** — apply to any story that touches `happenstance-testkit`. Each is a rule this
repository has already been burned by:

- **The rule can fail** — a named wrong implementation exists in the mutant registry and this rule
  rejects it. A rule no adapter can fail is decorative.
- **No literal position values** — the specification permits gaps, so asserting `[1, 2, 3]` converts a
  `MAY` into a `MUST` without an ADR (CF-6).
- **No clock** — a conformance rule does not read time (CF-33).
- **A changelog entry** — naming the defect the rule detects (CF-29).

## PR boundary

The paths this story is allowed to touch, as globs. `redkiln verify --grain story` reads the first
fenced block under this heading and fails on any file changed outside it — so this replaces a
self-reported list of touched files with one computed from git.

Write the narrowest set that is honestly true. A boundary widened to make a failing gate pass is a
boundary that has stopped meaning anything; widen it in the spec, deliberately, or split the story.

Include the story's own backlog folder when it will carry a ledger or report, and remember that
`spec/SPECIFICATION.md` is source here even though rustc never opens it.

```
crates/<crate>/src/**
crates/<crate>/tests/**
.bklg/<initiative>/<project>/<story>/**
```

## Error Conditions

- EC-001:

## Non-Functional

- NF-001:

## Anchors (progressive disclosure)

The load-bearing deeper artifacts — real repo/.kb paths, never inlined in bulk. Each anchor is SIGNPOSTED and
bound to the AC it serves so just-in-time retrieval is reliable, not discretionary (RFC §6.8/D7). One row each:

| Anchor (real path) | Why it is load-bearing | When to open | Serves AC |
| ------------------ | ---------------------- | ------------ | --------- |
|                    |                        |              |           |

## Out of Scope

What this work explicitly does not cover.
