# Contributing

## Before you start

Read [`docs/adr/`](docs/adr/). A handful of decisions shape everything else: the
two-flavour async ports, opaque payloads, the edition and MSRV, and the crate
naming. Changing one is fine — but it means writing a new ADR that supersedes
the old one, not working around it in code. ADR-0002/ADR-0005 and
ADR-0005/ADR-0006 are worked examples of that: the superseded body is left
factually intact rather than rewritten.

**Some ADRs are marked provisional.** ADRs 0001, 0003 and 0004 were authored in
one sitting alongside the initial scaffold, before the code they constrain
existed — and ADR-0002 was superseded thirty-two minutes after it was written.
Each provisional ADR states what would have to happen for it to become
precedent. Until that happens, treat it as a recorded intention: contradicting
one still needs a superseding ADR, but you do not owe deference to a decision
the code has not yet voted on. Do not let an unpublished API surface, or an
MSRV nobody depends on, decide a design question on its own.

## The gate

```console
cargo xtask ci
```

That is the whole thing: formatting, clippy with `-D warnings`, tests, the
`wasm32-unknown-unknown` build of `happenstance-core` and its feature powerset,
documentation — with `--all-features`, again with `--no-default-features`, and
once more on nightly under `--cfg docsrs` when a nightly toolchain is present —
specification traceability, a `cargo package --list` assertion that every
publishable crate ships both licences and a README, and, when the tools are
installed, `cargo hack` feature-powerset and `cargo deny`. It is defined once in
`xtask/src/main.rs`, and CI runs exactly the same command. If it passes locally,
it passes on CI.

Three of those need a word on why they exist, because each was added after
something passed that should not have. The docs build runs twice because a broken
intra-doc link is a hard rustdoc error rather than a warning, and three links
resolved only with `--all-features` on — the `no_std` configuration had been
failing for as long as the gate had existed, and nothing built it. The
`cargo package --list` assertion exists because `cargo publish --dry-run` does
not warn about a missing licence file; the first time you learn is when the crate
is on crates.io and cannot be edited. And specification traceability
(`cargo xtask spec-trace`) checks that `SPECIFICATION.md`'s cross-references
still resolve, on the principle that a specification whose citations have rotted
is worse than one that never made any, because it reads as though it is backed by
tests.

The plain `wasm32` check is load-bearing rather than decorative: it is the only
thing keeping the `!Send` port flavour honest until a Cloudflare adapter exists.
It stays mandatory and separate from the `wasm32` powerset beside it precisely so
that the constraint-1 guard cannot become skippable. If you add a step to the
gate, note that `cargo xtask wasm` looks its step up by **name** and not by
index, deliberately — an index is silent about what it selects, and getting it
wrong leaves that check running nothing while still printing green.

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
   happenstance_testkit::event_store_conformance!(MyStore::new());
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

Pull requests run an extra `cargo-semver-checks` job over all three publishable
crates, with `--baseline-rev` pointed at the commit the branch started from. A
breaking change is fine — an accidental one is not.

Know what that job proves and what it does not. It proves *this pull request*
does not break the API it branched from. It proves nothing about the last
released version: a break merged two pull requests ago is part of the baseline
and so is invisible. The registry baseline that would catch it is not available
yet — the three reserved names sit at `0.0.0`, and Cargo treats every `0.0.x`
version as incompatible with every other, so there is no compatible predecessor
to compare against. Phase 12 keeps both baselines once a real `0.1.0` exists.

A separate weekly job runs `cargo deny check advisories` and nothing else. A new
advisory against a dependency nobody has touched is the only failure that arrives
without a commit to trigger it, so it gets a schedule; licences and bans change
only when a manifest does, and the gate already covers those on every push.

## Licensing of contributions

Unless you state otherwise, any contribution you intentionally submit for
inclusion in this project — as defined in Apache-2.0 — is licensed under
`MIT OR Apache-2.0`, with no additional terms or conditions.

This is the Rust ecosystem's "inbound equals outbound" convention: what you send
in is licensed on the same terms the project ships out. There is no CLA and
nothing to sign. It exists so the project can keep dual-licensing without having
to track down past contributors for permission — which is a problem that is
trivial to prevent and expensive to fix, because the fix is asking every
contributor you have ever had.

Copyright in the project is held by Wet Ink Corporation (see
[`LICENSE-MIT`](LICENSE-MIT)); you keep the copyright in what you write, and this
section is the licence you grant, not an assignment.
