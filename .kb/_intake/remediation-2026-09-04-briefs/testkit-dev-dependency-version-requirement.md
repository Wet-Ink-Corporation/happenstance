# Does `[workspace.dependencies]` keep its `version` key on `happenstance-testkit`, now that every consumer of it is a dev-dependency?

Record: **U-2-testkit-dev-dependency-version**. Source:
`references/evaluation/review-pre-publication-2026-09-03.md:2241-2278`. Repository read
in the `lane/sqlite-ceilings` worktree at `bd11598`.

**This brief did not get the author → two-critic → revision pass the original thirteen
had.** It was written by the lane implementing U-2, in the same session as the change it
describes. Read it with that discount applied.

---

## Why this is owed

U-2's own Remediation names the choice and declines to make it:

> The finding sits on `crates/happenstance-sqlite/Cargo.toml:51`; the version requirement
> it inherits is at `Cargo.toml:41` and is load-bearing for nothing in the workspace,
> since every other reference to the testkit is a dev-dependency too. Which of the two
> lines moves is a real choice — the per-crate spelling matches the precedent that
> already exists and is documented, the root spelling fixes it once for every future
> adapter.

**The lane took the per-crate line and could not have taken the other**, for two
independent reasons: the root manifest is outside this lane's writable surface, and the
per-crate spelling is the one with a written precedent eleven lines long
(`crates/happenstance/Cargo.toml`, NF-006). That is a scope fact, not an argument, and
it is stated first so the recommendation below is not read as one.

## What is true today, after this lane

`crates/happenstance-sqlite/Cargo.toml` reads
`happenstance-testkit = { path = "../happenstance-testkit", features = ["proptest"] }`,
and the packaged manifest carries no `happenstance-testkit` dev-dependency at all —
checked, not assumed: `cargo package -p happenstance-sqlite --no-verify` produces a
`Cargo.toml` whose only `[dev-dependencies.*]` table is `tokio`.

Three facts remain.

1. **`Cargo.toml`'s `[workspace.dependencies] happenstance-testkit` still carries
   `version = "0.2.0-alpha.1"`.**
2. **Every reference to the testkit in this workspace is a dev-dependency.**
   `crates/happenstance-cloudflare/Cargo.toml` (`happenstance-testkit.workspace = true`),
   `crates/happenstance-postgres/Cargo.toml` (the same), `crates/happenstance/Cargo.toml`
   (path-only), `examples/outside-projection-adapter/Cargo.toml` (`version` + `path`,
   deliberately, because that crate exists to stand outside the workspace's conveniences)
   and now this crate. So the root `version` key protects nothing that would otherwise be
   unversioned.
3. **`happenstance-cloudflare` has the defect this lane just fixed one crate over.** It
   is in `PUBLISHABLE`, it is unpublished, and it inherits the same version requirement.
   It is another lane's crate and was not touched.

## Options

### Option A — leave the root key; fix each adapter's spelling as it joins the release set

**What it costs.** Every future adapter starts with the coupling and has to be
remembered out of it. `happenstance-cloudflare` is the outstanding instance and is a
0.2.0 publish target.

**What it buys.** The root key stays a single source of truth in shape even where it is
inert, so nobody reading `[workspace.dependencies]` has to know that one entry is
special. And a *future* non-dev dependency on the testkit — there is none today, and
none is planned — would inherit a version without anyone noticing its absence.

### Option B — drop `version` from the root `[workspace.dependencies]` entry

**What it costs.** The root manifest's own comment (`Cargo.toml:34-38`) calls a stale
version key *"the highest-cost line in the file to get wrong"*, and it is about
`happenstance-core` and `happenstance`, which are genuine registry dependencies. Removing
the testkit's key makes one entry in that table shaped differently from its neighbours,
which needs its own comment or it reads as the oversight the comment above warns about.
It also means a future non-dev dependency on the testkit publishes *unversioned*, which
cargo refuses at publish time rather than silently — so the failure is loud, but it is a
failure.

**What it buys.** The coupling cannot be reintroduced by inheritance, in any crate, ever.
`happenstance-cloudflare` is fixed without touching it. Every future adapter is fixed
before it exists.

**Semver.** None either way. It is a release-order constraint on the maintainer.

### Option C — a gate step

`xtask/src/package.rs`'s `reconcile` already holds the `PUBLISHABLE` set honest in both
directions. A step asserting that no publishable crate's dev-dependency on the testkit
carries a resolvable version would generalise `tests/front_page.rs`'s new per-crate
check. It is the only option that catches the *next* adapter without anybody
remembering.

**What it costs.** `xtask/` is a gate surface with its own owner and its own bar
(`standards/rust/80-the-gate.md`, `81-checks-that-cannot-be-types.md`), and this lane had
no writable claim on it.

## Recommendation

**Option B, then Option C, and neither is urgent.**

B is one line and removes the mechanism rather than its instances; C is what stops the
per-crate check from being one crate's private discipline. A did the job for this crate
and is not a policy.

**The strongest argument against, stated in its own words.** *Option A is what the
workspace already chose, deliberately, one crate over: `crates/happenstance/Cargo.toml`
did not move the root key when it had exactly this problem, and it wrote eleven lines
explaining the per-crate spelling instead. Repeating a decision is not a slip. Changing
the root key now says the earlier judgement was wrong, on the evidence of a second
instance and nothing else — and two instances of a documented pattern is what a pattern
looks like.* That is a fair reading, and it is why B is recommended rather than assumed:
what moves it is fact 2, which nothing in the earlier decision states — the root key is
inert **today**, and the earlier lane did not check that.

## Cost of delay

Low but non-zero and dated. `RUNBOOK.md`'s phase 12 publish-order item already fixes
`core → testkit → happenstance → sqlite`, which masks this for `0.2.0` specifically —
which is exactly why U-2 asked for it to be routed now rather than found later. The next
testkit-only minor bump after `0.2.0` is when `happenstance-cloudflare` finds out.

## What this does not settle

- Whether `happenstance-cloudflare`'s spelling moves, and by whom. It is the outstanding
  instance and it is in `PUBLISHABLE`.
- Whether `examples/outside-projection-adapter`'s `version` + `path` spelling is right.
  It probably is — that crate exists to stand where a stranger stands — but nobody has
  written down that it is deliberate, and it is the one entry that would look like a
  counter-example to whoever applies Option B.
