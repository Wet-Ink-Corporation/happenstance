# Contributing

## Before you start

Read [`docs/adr/`](docs/adr/). Four decisions shape everything else: the
two-flavour async ports, the crate naming, opaque payloads, and the edition and
MSRV. Changing one is fine — but it means writing a new ADR that supersedes the
old one, not working around it in code.

## The gate

```console
cargo xtask ci
```

That is the whole thing: formatting, clippy with `-D warnings`, tests, the
`wasm32-unknown-unknown` build of `eventum-core`, documentation, and — when the
tools are installed — `cargo hack` feature-powerset and `cargo deny`. It is
defined once in `xtask/src/main.rs`, and CI runs exactly the same command. If it
passes locally, it passes on CI.

Optional tools, if you want the full gate locally:

```console
cargo install cargo-hack cargo-deny --locked
```

## Writing an adapter

1. Implement `SendEventStore` if your store can be shared across threads — every
   native adapter can. Implement `EventStore` only when you genuinely cannot, as
   on `wasm32`. Implementing the former gives you the latter for free.
2. Invoke the conformance suite:
   ```rust
   eventum_testkit::event_store_conformance!(MyStore::new());
   ```
3. Make it pass.

Step 3 is not a formality. An adapter that compiles but has not run the suite is
not an adapter, and will not be merged as one. If a rule looks wrong, say so and
fix the rule — a bad rule costs every future adapter author a day.

## Conformance rules

When adding one:

- Trace it to a MUST in the [specification](https://dcb.events/specification/),
  or to a property an adapter can plausibly get wrong.
- Put the reason in the assertion message. An adapter author reading a failure
  should learn what rule they broke, not just that something was `false`.
- **Never assert on literal position values.** The specification permits gaps in
  the sequence, so `assert_eq!(positions, [1, 2, 3])` fails a perfectly
  conformant adapter. Compare against positions the store actually assigned.

## Style

- Public items are documented. Fallible functions get an `# Errors` section.
- No `unwrap`/`expect` in library code. Test modules opt out locally with
  `#![allow(clippy::unwrap_used)]`.
- No `anyhow` in library crates; `xtask` and examples may use it.
- Comments explain why, not what.
- Prefer a runnable doctest to a described example.

## Commits and pull requests

Keep the reasoning in the commit message. A diff shows what changed; the message
should say what constraint made that the right change.

Pull requests run an extra `cargo-semver-checks` job against the published
crates. A breaking change is fine — an accidental one is not.
