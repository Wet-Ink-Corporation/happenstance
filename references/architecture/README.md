# The workspace, drawn

Evidence, not a contract. Nothing here binds anything; where this drawing and
[`spec/SPECIFICATION.md`](../../spec/SPECIFICATION.md) disagree, the clause wins,
and where it and [`RUNBOOK.md`](../../RUNBOOK.md) disagree about *status*, see the
caveat at the bottom.

| File | What it is |
|---|---|
| `workspace.html` | the diagram, self-contained — no network, no CDN, opens from `file://` |
| `workspace.json` | the source it was rendered from |

## What it draws

One primary path, traced end to end: `Application → happenstance →
happenstance-core → happenstance-sqlite → SQLite`, labelled `commit()`,
`read → decide → append`, `rusqlite`, and closed by a dashed
`ConditionViolated → re-decide` back into the typed layer. That return edge is the
command loop, which is why it earned an edge rather than a footnote.

Three boundaries, and they are the reason the drawing exists rather than a list of
crates:

* **One binary.** The typed layer, the contract and all four adapters link into
  the consumer's process. There is no hop between them, and a diagram that put
  boxes and arrows between crates without saying so would imply one.
* **The conformance gate**, nested inside it around the four adapters. An adapter
  that compiles but has not run [`happenstance-testkit`](../../crates/happenstance-testkit)
  is not an adapter, and the boundary carries that rather than four more edges.
* **Operator-owned storage.** SQLite, PostgreSQL, the Durable Object and Neon.
  Schema, credentials and durability stop being ours at the adapter edge; this is
  the only real trust line on the page.

Two authoring calls worth knowing before editing the source:

**The four `happenstance-core → adapter` edges carry no label.** The
distinguishing fact is which flavour each implements, and it lives in the node
sublabels — `SendEventStore · serialising`, `EventStore · !Send, wasm32` — because
four labels in one 70px corridor collided, and because the flavour is a property
of the adapter rather than of the call.

**`happenstance-testkit` connects to the contract, not to the adapters.** It
*depends on* `happenstance-core` and *gates* the adapters; the gating is the
boundary's job.

Present in the tree and deliberately off the page: `MemoryEventStore` and
`MemoryProjectionStore` (inside the contract crate), the projection runner,
[`happenstance-sync`](../../crates/happenstance-sync) — a third port, not an
adapter — and [`happenstance-ladybug`](../../crates/happenstance-ladybug).

## Reproducing it

Rendered by [Archify](https://github.com/tt-a1i/archify) at the `showcase` quality
profile. The renderer is deterministic: re-rendering `workspace.json` produced the
committed `workspace.html` byte for byte.

```console
archify deliver architecture workspace.json workspace.html \
    --quality showcase --repo-root <this repository>
```

| | SHA-256 | Bytes |
|---|---|---|
| `workspace.json` | `293a80ccd7675f1c0062b6da937dcfd8da6e42a0500f0a47fe8a5e9b4f7c821a` | 10,065 |
| `workspace.html` | `3a0de2acf97abe38c49b69cd8a4be76828672fd821224e54f9e0a3b1bc6c8052` | 736,642 |

`--repo-root` is not optional here. The source declares nine `sources` references
into this repository, and the renderer refuses to draw until it has checked every
one of them against the revision named in `meta.repository` —
`910c5ac611660dd02ca5daaf76895518d3deaf58`. That is what stops the drawing from
citing a file that has since moved.

Nine artifact checks pass with zero composition errors and zero warnings, and the
page was checked in a real browser at 1440×900, 1600×1000, 1920×1080 and
2048×1320 in both themes. None of that is in `cargo xtask ci`, and it should not
be: the gate's business is the library, and this is a picture.

## The status caveat

The tags on the adapter nodes — `suite green`, `101/101 live`, `skeleton` — were
read off the crates' own module documentation on
`lane/postgres-neon-stores`, which is **ahead of `RUNBOOK.md`**. That file's status
table still carries phase 10 as `not started` while
[`happenstance-postgres`](../../crates/happenstance-postgres/src/event_store.rs)
claims 101 of 101 gated tests against a live PostgreSQL 17.10. Both statements are
in the tree; only one of them has run a server. The runbook is the thing to
correct, and correcting it is not this drawing's job.
