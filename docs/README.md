# Documentation

This directory is the source of **[the happenstance guide](https://wet-ink-corporation.github.io/happenstance/guide/)**,
which is easier to read on the site. Every Rust example on these pages is
compiled against the real crates by the repository's tests, so an example that
stopped working would fail the build before it reached a reader.

## New to event sourcing? Read these in order

| Page | Read it at |
| --- | --- |
| Events, and the systems built on them | [`event-driven-systems.md`](event-driven-systems.md) |
| Event sourcing, and CQRS | [`event-sourcing-and-cqrs.md`](event-sourcing-and-cqrs.md) |
| Aggregates, and the boundary drawn in advance | [`aggregates.md`](aggregates.md) |
| Dynamic consistency boundaries | [`what-is-dcb.md`](what-is-dcb.md) |
| Your first encounter: watch a write get refused | [`first-encounter.md`](first-encounter.md) |

## Everything else

| Page | Read it at |
| --- | --- |
| The canonical DCB program, worked end to end | [`read-the-worked-example.md`](read-the-worked-example.md) |
| Where the aggregate's consistency boundary went | [`boundary-not-aggregate.md`](boundary-not-aggregate.md) |
| Carrying the worked example's rule into your own domain | [`carry-your-invariant.md`](carry-your-invariant.md) |
| Appending under a condition | [`append-conditions.md`](append-conditions.md) |
| Retrying a command without applying it twice | [`retry-a-command.md`](retry-a-command.md) |
| Rebuilding a read model from the log | [`rebuild-a-read-model.md`](rebuild-a-read-model.md) |
| Choosing a store for your deployment | [`choose-a-store.md`](choose-a-store.md) |
| Running the event store inside a Durable Object | [`run-on-a-durable-object.md`](run-on-a-durable-object.md) |
| What to read before you write an adapter | [`adapter-reading-order.md`](adapter-reading-order.md) |
| Passing the conformance suite with your adapter | [`pass-conformance-suite.md`](pass-conformance-suite.md) |
| Moving your code from 0.3 to 0.4 | [`upgrading-to-0-4.md`](upgrading-to-0-4.md) |
| Fences the compiler never sees (a fixture for the build, not a guide page) | [`text-fences.md`](text-fences.md) |

## For contributors

This directory is the **narrative tree**: user documentation only. The rest of
the project's records live elsewhere, and route from here.

| Looking for | It is at |
| --- | --- |
| The numbered clauses that are true now | [`spec/SPECIFICATION.md`](../spec/SPECIFICATION.md) |
| The end-to-end cases, as observable behaviour | [`spec/E2E-CASES.md`](../spec/E2E-CASES.md) |
| The plan, and how far it has got | [`RUNBOOK.md`](../RUNBOOK.md) |
| House style for Rust in this repository | [`standards/rust/`](../standards/rust/README.md) |
| What makes a narrative page teach, and the rules a page author follows | [`standards/pages/`](../standards/pages/README.md) |
| Measurements — position visibility, the wire format, a rustc ICE | [`experiments/`](../experiments/) |
| The reviews and research the design rests on | [`references/evaluation/`](../references/evaluation/README.md) |
| Six deployments the contract was walked against | [`references/scenarios/`](../references/scenarios/README.md) |
| What the six skeletons told the type checker | [`references/adapter-shapes.md`](../references/adapter-shapes.md) |
| The decisions, and why each was taken | [`.kb/decisions/`](../.kb/decisions/) — the full records are [`references/adr/`](../references/adr/) |
| What is deliberately still unsettled | [`.kb/open-questions/`](../.kb/open-questions/) |

Four trees are read by the gate rather than only by people: `cargo xtask
spec-trace` parses `spec/SPECIFICATION.md` and `spec/E2E-CASES.md` by path;
`cargo xtask lint-constitution` plus `cargo test -p xtask --doc` read every file
under `standards/rust/`; the narrative pages in this directory are registered
one by one in `xtask/src/narrative.rs`, where `cargo test -p xtask --doc` compiles
their examples under the mandatory `cargo xtask narrative-doctests` step; and
`standards/pages/` is pinned the same way — by path, in `xtask/src/lint_pages.rs`
— and the mandatory `cargo xtask lint-pages` step walks that directory, so a
dangling index link, an over-budget atom or a generated index that has fallen
behind the corpus fails `cargo xtask ci` today. That fourth reader also reads
*this* directory, from the other side: it asks of every page here which one
reader's question the page answers, and fails by file and line when the answer is
absent, doubled or outside the closed set. Moving any of them
means editing `xtask/src/` in the same change — which is the point of pinning
them by path rather than by convention.

This directory has a second gate-side reader: `cargo xtask narrative` opens it as
*files* rather than as doctests, and fails by name when a page here is missing
from that harness, when a registration outlives the page it named, or when the
directory itself has moved out from under the constant that pins it.

## What belongs here

Guides, tutorials, how-to pages, and reference material addressed to someone
*using* the library. If the reader you have in mind is a contributor, a reviewer,
or your future self reconstructing why a decision was taken, the page belongs in
one of the trees above instead.
