# Documentation

**This directory is for user documentation** — what someone who has run `cargo add
happenstance` needs in order to use it. It is deliberately near-empty: the crate's
documentation currently lives in its rustdoc and its README, and nothing has yet
been written that belongs here instead.

Everything else that used to live under `docs/` was moved out, because a directory
holding both "how to use this library" and "the 9,000-line normative specification"
serves neither reader.

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

Two of those are read by the gate rather than only by people: `cargo xtask
spec-trace` parses `spec/SPECIFICATION.md` and `spec/E2E-CASES.md` by path, and
`cargo xtask lint-constitution` plus `cargo test -p xtask --doc` read every file
under `standards/rust/`. Moving either tree means editing `xtask/src/` in the same
change — which is the point of pinning them by path rather than by convention.

## What belongs here

Guides, tutorials, how-to pages, and reference material addressed to someone
*using* the library. If the reader you have in mind is a contributor, a reviewer,
or your future self reconstructing why a decision was taken, the page belongs in
one of the trees above instead.
