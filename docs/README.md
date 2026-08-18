# Documentation

**This directory is for user documentation** — what someone who has run `cargo add
happenstance` needs in order to use it. It is the **narrative tree**: the Rust
examples on the pages below are compiled by `cargo xtask ci` against the real
workspace crates, so an example that stopped compiling fails the gate rather than
a reader.

Everything else that used to live under `docs/` was moved out, because a directory
holding both "how to use this library" and "the 9,000-line normative specification"
serves neither reader.

| Page | Read it at |
| --- | --- |
| Appending under a condition | [`append-conditions.md`](append-conditions.md) |

| Looking for | It is at |
| --- | --- |
| The numbered clauses that are true now | [`spec/SPECIFICATION.md`](../spec/SPECIFICATION.md) |
| The end-to-end cases, as observable behaviour | [`spec/E2E-CASES.md`](../spec/E2E-CASES.md) |
| The plan, and how far it has got | [`RUNBOOK.md`](../RUNBOOK.md) |
| House style for Rust in this repository | [`standards/rust/`](../standards/rust/README.md) |
| Measurements — position visibility, the wire format, a rustc ICE | [`experiments/`](../experiments/) |
| The reviews and research the design rests on | [`references/evaluation/`](../references/evaluation/README.md) |
| Six deployments the contract was walked against | [`references/scenarios/`](../references/scenarios/README.md) |
| What the six skeletons told the type checker | [`references/adapter-shapes.md`](../references/adapter-shapes.md) |
| The decisions, and why each was taken | [`.kb/decisions/`](../.kb/decisions/) — the full records are [`references/adr/`](../references/adr/) |
| What is deliberately still unsettled | [`.kb/open-questions/`](../.kb/open-questions/) |

Three trees are read by the gate rather than only by people: `cargo xtask
spec-trace` parses `spec/SPECIFICATION.md` and `spec/E2E-CASES.md` by path;
`cargo xtask lint-constitution` plus `cargo test -p xtask --doc` read every file
under `standards/rust/`; and the narrative pages in this directory are registered
one by one in `xtask/src/narrative.rs`, where `cargo test -p xtask --doc` compiles
their examples under the mandatory `cargo xtask narrative-doctests` step. Moving
any of the three means editing `xtask/src/` in the same change — which is the
point of pinning them by path rather than by convention.

This directory has a second gate-side reader: `cargo xtask narrative` opens it as
*files* rather than as doctests, and fails by name when a page here is missing
from that harness, when a registration outlives the page it named, or when the
directory itself has moved out from under the constant that pins it.

## What belongs here

Guides, tutorials, how-to pages, and reference material addressed to someone
*using* the library. If the reader you have in mind is a contributor, a reviewer,
or your future self reconstructing why a decision was taken, the page belongs in
one of the trees above instead.
