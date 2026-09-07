# Is `&'static str` the projection batch's final SQL seam, or does the parameterised path get a minted statement type?

Record: **X-4-projection-batch-sql-seam**. Source:
`references/evaluation/review-pre-publication-2026-09-03.md:2425-2472`. Repository read in
the `lane/sqlite-ceilings` worktree at `bd11598`; every `crates/happenstance-sqlite/src/`
path cited below is byte-identical to the review's pinned `56ef6c5` for the lines quoted,
except where this lane moved them, which is stated.

**This brief did not get the author → two-critic → revision pass the original thirteen
had.** It was written by the lane implementing X-4, in the same session as the change it
describes. It carries its own strongest objection and answers it, which is the form, but
nobody independent argued the other side. Read it with that discount applied.

---

## Why this is owed

X-4 found that `SqliteBatch::push` took `impl Into<String>` and that its entire doc
comment was one line — *"Queues a statement to run when the batch commits."* No
`# Security`, no mention of binding, and a type-level doc above it that framed free-form
SQL as the intended use and named `params` nowhere. A `grep` for
`inject|parameteris|parameteriz|untrusted|trust boundary` over the crate's `src/`, its
README and `docs/` returned nothing.

That is the only place in the workspace a consumer is handed a SQL-text seam, and the
values that flow through it are exactly the bytes the library guarantees it does not
inspect: ADR-0003 makes payloads opaque `Bytes`, forwarded and validated by nothing.

**The lane implemented the type half rather than the paragraph half, and the reason is
this repository's own doctrine.** `standards/rust/70-rustdoc-obligations.md`, RS-70-5:
*"Nothing in the gate reads prose."* CLAUDE.md: *"A rule that no adapter can fail is
decorative."* A `# Security` paragraph on `push` would have been a control nothing
enforces, and the audit's own Remediation says so in the same words.

What is **not** settled by that, and is the question here: whether `&'static str` is the
shape the seam keeps, or a way-station.

## What is true today, after this lane

`crates/happenstance-sqlite/src/projection_store.rs:473`:

```rust
    pub fn push(&mut self, sql: &'static str, params: impl IntoIterator<Item = Value>) {
        self.push_raw_sql(sql, params);
    }
```

and, beside it, `push_raw_sql`, which keeps the old `impl Into<String>` and is named so
that reaching for it is a decision. A `compile_fail,E0308` doctest on `push` holds the
narrowing; the fence is the whole of the enforcement, and it is checked by
`cargo test -p happenstance-sqlite --all-features`.

Three facts bound the options.

1. **Every caller in the workspace already passed a literal.** The crate's own
   `probe_write` and `probe_delete_all`, and `examples/transfers-on-sqlite/src/main.rs:562`.
   None changed. The narrowing cost this workspace nothing, which is evidence about how
   the seam is used and not proof that no consumer needs more.
2. **The surface is opt-in and disclaims semver.** `push` lives behind
   `projection-store`, which left `default` under PS-3's verdict, and whose manifest
   comment says *"**Enabling this enables an unfrozen port** … `ProjectionStore` is
   provisional: PS-2's bar is unmet"* (`crates/happenstance-sqlite/Cargo.toml`). A caller
   reaching `push` has already opted into an unfrozen port. That is why the audit called
   this the one entry in its theme whose window is *soft*.
3. **`happenstance-sqlite` is not on crates.io.** A `0.0.0` placeholder is. So the
   narrowing was free at the moment it was made, and a second narrowing after `0.2.0` is
   not.

## Options

### Option A — keep `&'static str` + `push_raw_sql` (what landed)

**Cost to a caller.** Nothing for a literal. A statement whose shape is computed — an
`IN (…)` list sized by the number of keys — moves one identifier, to `push_raw_sql`.

**Cost to an adapter author.** None; `SqliteBatch` is this adapter's own type, not a
port type. Nothing in `happenstance-core`'s `ProjectionStore` names it.

**Semver.** The narrowing is breaking and the compiler sees it (E0308). `push_raw_sql`
is additive.

**What it does not buy.** `&'static str` proves *provenance*, not *shape*. A
`const BAD: &str = "… WHERE k = 'x' OR 1=1";` is still accepted, correctly — it is in the
source. And `Box::leak(format!(…).into_boxed_str())` defeats it in one line for anyone
determined to; the type is a guard rail, not a proof.

### Option B — a `Statement` newtype minted by a macro from a literal

`sql!("INSERT INTO …")` yields a `Statement`, and `push` takes one. The same guarantee,
plus a place to hang future obligations — a parameter-count check against the
placeholders in the text, for instance, which `&'static str` has nowhere to live.

**Cost to a caller.** One import and one wrapper at every call site, forever, for a
guarantee `&'static str` already gives today. The macro is exported surface
(`standards/rust/41-declarative-macros.md` applies in full: `$crate` paths, a hygiene
story, a `compile_fail` per refusal).

**Semver.** Breaking, again, if it lands after Option A. Two narrowings.

### Option C — leave the seam alone and document it

Rejected before it was written up, and recorded so the option is visible rather than
absent: it is the paragraph-only fix the audit's own Remediation calls a control nothing
enforces.

## Recommendation

**Option A, and then stop until PS-2's bar is met.**

The seam's final shape belongs with whoever freezes `ProjectionStore`, exactly where the
audit routed it: *"a signature narrowed twice is worse than one narrowed once."* Option A
is the narrowing that was free today and that costs a consumer nothing to adopt, because
no consumer exists and every in-tree caller already complies. Option B's extra guarantee
— a placeholder/parameter arity check — is real, and it is also the kind of thing a
frozen port should decide once for *every* projection adapter rather than once for the
SQLite one; `NeonProjectionStore` and `LadybugProjectionStore` have the same seam in
their own dialects.

**The strongest argument against, stated in its own words.** *The window closes at
`0.2.0` and Option A spends it on the weaker of the two guarantees. If a statement type
is where this ends up, the honest move is to pay the break once, now, while nobody is
pinned — and PS-2's bar has been unmet for two phases with no named event that meets it,
so "wait for the freeze" is indistinguishable from "never".* That is a fair reading. What
answers it is the second half of fact 2: the manifest already says the surface behind this
flag makes no semver promise, so the break is available after `0.2.0` at a cost the rest
of this crate's public surface does not enjoy. The window is soft here and hard
everywhere else in the same audit theme, which is why this one can wait and the others
could not.

## Cost of delay

Low, and asymmetric in a useful direction. Option A is landed, so the injection path is
closed today; delaying B costs an arity check nobody has asked for. Delaying *A* would
have cost the whole finding, which is why it was not deferred.

## What this does not settle

- Whether `PendingStatement::sql` should stay `Box<str>` or become the minted type.
- Whether the same narrowing is owed by `NeonProjectionStore`'s and
  `LadybugProjectionStore`'s write vocabularies. ADR-0017 settled that the batch carries
  no universal write vocabulary, so this cannot be answered once for all three by
  construction — but the *obligation* could be stated once, and is not.
- Whether `metadata` and `data` reaching a projection at all should carry a
  documented trust boundary in `happenstance-core`. This brief is about one adapter's
  seam, not about where the contract says untrusted bytes stop being untrusted.
