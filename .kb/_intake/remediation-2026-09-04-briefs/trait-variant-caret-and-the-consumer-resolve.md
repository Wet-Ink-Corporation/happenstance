# The two-flavour public API is emitted by a caret-resolved proc macro, and every guard on it runs `--locked`. Does the requirement narrow, or does CI learn to float?

Short answer up front: **neither half of the fix is inside this lane's writable
surface, so this brief records the measurement and the routing rather than a
change.** The one-line half — a scope qualifier on a sentence in
`crates/happenstance-core/src/store.rs` that is true of this workspace and false
of the published artifact — is the cheapest correction in this directory and is
owed to whoever holds that crate next.

**This brief did not get the author → two-critic → revision pass the original
thirteen had.** Read it with that discount.

---

## Why this is owed

`SendEventStore`, `SendProjectionStore` and the blanket impl that
`CLAUDE.md`'s binding constraint 4 rests on are **not written in this
repository**. They are emitted at compile time by `trait_variant`, and the
manifest requirement is a caret:

```toml
# Cargo.toml:130
trait-variant = "0.1.3"
```

`crates/happenstance-core/src/store.rs` states the exposure precisely, and then
mitigates only the half inside this tree:

> Parsed rather than pinned in the manifest: `trait-variant = "0.1.3"` is a caret
> requirement, so `0.1.4` would resolve without the manifest changing.

and twenty-seven lines later:

> repository can observe the expansion, so the version is asserted instead — the
> gate builds `--locked`, so the resolved version cannot move without someone
> changing it deliberately

That second sentence carries **no scope qualifier**. It is true of this
workspace's gate and false of the published artifact.

---

## What is true today, measured in this working tree

- `Cargo.toml:130` is `trait-variant = "0.1.3"` — a caret requirement.
- `xtask/src/main.rs` contains **32** `"--locked"` occurrences (31 at the review's
  commit; this lane's new gate step added one).
- `.github/workflows/ci.yml` has a `schedule:` trigger — `cron: "17 7 * * 2"` —
  and the jobs it can drive are `gate`, `backlog`, `msrv`, `semver` and
  `advisories`. None of them resolves the dependency graph freshly; the weekly
  trigger exists for `advisories`.
- Every existing guard on the derivation is `#[cfg(test)]`, so all of them run
  against the locked version and none can observe a different one.

`Cargo.lock` does not travel to a consumer of a library crate. Three of these
crates are already on crates.io at `0.2.0-alpha.1`, so a consumer whose resolve
can float is possible **today**, not at `0.2.0`.

---

## The question, and the options

**Q1. Does the requirement narrow to `=0.1.3`?**

- **Option A — narrow it.** Cost: a manual bump for every patch release of
  `trait-variant`, and a hard conflict for any consumer who also depends on it at
  a different version — which for a proc-macro crate in a public API's expansion
  path is not hypothetical.
- **Option B — leave the caret and add a derivation contract compiled into
  `happenstance-core`'s lib** (not `#[cfg(test)]`), so a shape change fails at the
  consumer's build with a message naming the cause rather than compiling to a
  different flavour relationship.
- **Option C — leave it and correct the sentence.**

**Q2. Does a floating-resolve job join CI?**

A `floating-deps` job on the `schedule:` trigger that already exists, resolving
without the lock file, is non-blocking by construction and green today. Note that
`rm Cargo.lock` is the wrong verb for it — `cargo update -p trait-variant` or a
`--ignore-rust-version`-style fresh resolve in a scratch copy is, so the
committed lock is not mutated in the runner's workspace.

**Recommendation: C now and B next, with Q2 taken whenever CI is next opened.**
The strongest argument against leaving the caret: the exposure is not a
hypothetical about a future `0.1.4` — it is that the workspace has *no* guard
that can see a different version, so the first observation of a shape change is a
consumer's build. Against *narrowing*: an `=` requirement in a library's public
manifest is a dependency-policy decision that constrains every downstream graph,
and this repository's own `standards/rust/50-dependency-hygiene.md` is where that
policy lives.

Neither half was taken here because both live outside this lane's writable
surface: `crates/` is held by another lane for the duration of this remediation,
and `.github/workflows/` is not `xtask/`.

---

## Cost of delay

Unbounded and out of this repository's control: it is priced by whenever
`trait-variant 0.1.4` publishes. The residual survives every option — a shape
change in a version nobody has published cannot be tested by anything.

---

## What this does not settle

Whether `standards/rust/50-dependency-hygiene.md` acquires a rule about
proc-macro crates whose expansion is public API. ADR-0001 and ADR-0035 own the
two-flavour derivation; this is their dependency, and the decision belongs with
them rather than with whoever notices.
