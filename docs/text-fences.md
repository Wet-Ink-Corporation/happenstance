# Fences the compiler never sees

> **Answers:** `explanation` — Which blocks on these pages does the gate never compile?

A fenced block tagged `text` is prose as far as the toolchain is concerned. The
gate compiles the Rust examples on the pages in this directory; it hands a
`text` fence to nobody, and the page checker permits the tag rather than
reporting it. So a Rust-looking block called `text` is neither compiled nor
flagged, and that is a hole this directory narrows rather than closes.

The fence below is Rust, and every sentence in it is **false** about this
library. It is here on purpose, and it is the demonstration that the limit is
real rather than a cautious sentence somebody wrote:

```text
let store = MemoryEventStore::new();
assert_eq!(store.len(), 7);
```

`MemoryEventStore::new()` returns an empty store, so a reader who copied that
block would find it fails on the first line that matters. Nothing in the gate
says so. The instrument that catches it is a reviewer reading the tag in the
diff — and, if the tag is wrong, the fence walk, which refuses an untagged
block outright because rustdoc compiles one as Rust whatever the author meant.

Tag a block `rust` when you want the compiler's opinion of it. Tag it `text`
only when the block is not Rust you are claiming anything about.
