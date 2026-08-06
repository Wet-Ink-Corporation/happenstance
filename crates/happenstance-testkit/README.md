# happenstance-testkit

The conformance suite for [happenstance](https://github.com/Wet-Ink-Corporation/happenstance)
event store adapters. Twenty-seven rules, each tracing to a MUST in the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/).

> **Status: early.** The suite runs and is green against the reference store, but
> it is measurably weaker than it needs to be — deliberately-broken adapters have
> passed it. Strengthening it is scheduled work, not an aspiration; see
> [the specification](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/docs/architecture/SPECIFICATION.md)
> §6, which states what the suite must itself be shown to fail.

## Use

```rust,ignore
happenstance_testkit::event_store_conformance!(MyStore::new());
```

That expands to one `#[tokio::test]` per rule, so a failure names the rule rather
than a line number. The expression is re-evaluated per test, so it must yield a
genuinely fresh, empty store each time.

## Why this exists as a published crate

A claim about behaviour is worth exactly as much as the test that checks it.
Publishing the suite means an adapter author outside this repository can make the
same claim on the same terms, and it means the reference store is measured by the
same instrument as everyone else.

Two rules govern what goes in it, both learned expensively:

**A rule no adapter can fail is decorative.** Before a rule is added, a plausible
wrong implementation it rejects has to be named — and written into this crate's
own `tests/`, so the rule is demonstrated to fail before it is trusted to pass.

**Never assert on literal position values.** The specification permits gaps, and a
conformant adapter may leave them. Rules compare against positions the store
actually assigned.

## Versioning

Adding a rule is a semver-*minor* change that can turn a passing adapter's CI red.
Treat that as a breaking change in practice and pin this crate exactly. Its
version is independent of the rest of the workspace for that reason.

## Licence

MIT OR Apache-2.0.
