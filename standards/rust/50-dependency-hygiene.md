# 50 — Dependency hygiene

> **Load when:** adding a crate to a manifest · `cargo deny check` failed ·
> choosing a version requirement · two majors of one crate in the graph · a
> member cannot turn a workspace default off · tempted to add a dev-dependency
> for four assertions
> **See also:** 51 (features and the ladder) · 52 (wasm32) · 80 (the gate) ·
> 90 (skeletons)

---

## RS-50-1. Pin every third-party version once, in `[workspace.dependencies]`, and justify it by counting nodes.

**Why.** Cargo unifies semver-compatible requirements onto one node and keeps
incompatible ones as two, so a requirement written in a member manifest is a
second place the graph can fork. `base64.workspace = true` makes the requirement
one string in one file.

**Do** — the requirement, and the measurement that chose it:

```toml
# `sqlx` already pulls base64 0.22.1 into this graph, and base64 has no
# transitive dependencies of its own: at "0.22" the addition is an edge to a
# node that already exists. Zero new crates.
base64 = { version = "0.22", default-features = false }
```

**Not** — resolves, compiles, and puts two majors of one crate in the graph.
`cargo deny check bans` sees it; `multiple-versions = "warn"` means it *reports*
rather than fails, and the gate still exits 0:

```toml
base64 = "0.23"   # in a member manifest, or as a newer workspace pin
```

**Rejects.** A member that wants the newer API adds its own line, everything
builds, and the only instrument is a warning inside a step that passes. The
consumer who pays is the one taking `happenstance-core/serde` without sqlx: the
measurement promised exactly one extra leaf crate and they now compile two
majors of it, discovered — if ever — in `cargo deny` output nobody reads.
`wildcards = "deny"` is the same lint family set to the other level, and it is
`deny` precisely because a `*` requirement cannot be assessed at all.

**Evidence.** `Cargo.toml:68 (zero new crates)` ·
`Cargo.toml:72 (a warning nobody reads)` ·
`deny.toml:23 (multiple-versions)` ·
[cargo-deny bans](https://embarkstudios.github.io/cargo-deny/checks/bans/cfg.html) *(checked 2026-08-09, rustc 1.97.1)*

## RS-50-2. Write `default-features = false` on the workspace's internal entries: a member can add a default back, never take one away.

**Why.** Measured on cargo 1.97.1: a member that inherits a workspace dependency
and writes `default-features = false` narrows nothing. Cargo **ignores the key
and warns** — *"`default-features` is ignored for `<dep>`, since
`default-features` was not specified for `workspace.dependencies.<dep>`, this
could become a hard error in the future"* — and the build succeeds. The reverse
direction, workspace `false` and member `default-features = true`, is accepted
without a warning, so the restrictive choice at the workspace level is the
reversible one.

**Do**

```toml
# workspace
happenstance-core = { version = "0.2.0", path = "crates/happenstance-core", default-features = false }
# member — states what it actually needs
happenstance-core = { workspace = true, features = ["std"] }
```

**Not** — permissive at the workspace level. The member's attempt to narrow it
resolves, builds, and keeps the defaults anyway. The only signal is a cargo
*manifest* warning, which the gate's `-D warnings` does not reach:

```toml
# workspace: happenstance-core = { path = "crates/happenstance-core" }
# member:    happenstance-core = { workspace = true, default-features = false }
# warning: `default-features` is ignored for happenstance-core, since
#          `default-features` was not specified for
#          `workspace.dependencies.happenstance-core`, this could become a hard
#          error in the future
```

**Rejects.** With defaults inherited, every adapter links `memory`, and `memory`
implies `std`. `happenstance-cloudflare` — the crate that exists to be the
`!Send`, single-threaded instrument — acquires the reference store it is meant
to be an alternative to, and the wasm32 build acquires `std` with it. The member
author who tries to fix it in their own manifest writes the line, watches cargo
accept it, and gets the defaults regardless — `-D warnings` reaches rustc and
clippy, not cargo's manifest warnings, so every step of the gate stays green.
The real fix is a workspace edit that changes what all eight crates resolve.

**Evidence.** `Cargo.toml:50 (Cargo forbids a member from)` ·
`crates/happenstance-core/Cargo.toml:48 (default = ["std", "memory"])` ·
[Cargo Book — inheriting a dependency from a workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-dependencies-table) *(checked 2026-08-09, rustc 1.97.1)*

## RS-50-3. `default-features = false` on an external crate is a claim about what its default pulls in — measure it.

**Why.** A default feature set is its author's decision about their median user,
inherited transitively into yours. Two pins here turn it off for measured
reasons: `rusqlite` 0.40's default adds `ffi-sqlite-wasm-rs`, which no native
adapter wants, and `postcard`'s default reaches `heapless` and then
`atomic-polyfill 1.0.3`.

**Do**

```toml
rusqlite = { version = "0.40", default-features = false, features = ["bundled"] }
postcard = { version = "1", default-features = false, features = ["use-std"] }
```

**Not** — both resolve and both compile. The second fails
`cargo deny check advisories` on RUSTSEC-2023-0089, an unmaintained crate for
which *"no safe upgrade is available"*:

```toml
rusqlite = "0.40"
postcard = "1"
```

**Rejects.** Nothing here is a type error, so the author's `cargo build` and
`cargo test` are both green and the manifest line looks like the shortest
correct spelling. `cargo deny` also runs weekly on a schedule with no commit
behind it, so the person who sees red is whoever opens CI on a Tuesday — and
the advisory names a crate three levels down that no manifest in this workspace
mentions.

**Evidence.** `Cargo.toml:91 (ffi-sqlite-wasm-rs)` ·
`Cargo.toml:102 (RUSTSEC-2023-0089)` ·
`.github/workflows/ci.yml:1150 (cargo deny check advisories)` ·
[RUSTSEC-2023-0089](https://rustsec.org/advisories/RUSTSEC-2023-0089.html) *(checked 2026-08-09, rustc 1.97.1)*

## RS-50-4. A licence rejection is a dependency choice, and the offender is rarely the crate you expect.

**Why.** `cargo deny check licenses` denies every licence not named in
`[licenses] allow`, and `deny.toml`'s `[graph] all-features = true` builds the
graph with every optional dependency present. Enabling a feature on a crate
already in the tree can therefore fail the gate without adding a line to any
`[dependencies]` table.

**Do** — no TLS backend chosen, and the measurement recorded beside the pin:

```toml
sqlx = { version = "0.8", default-features = false }
```

**Not** — compiles; fails `cargo deny check licenses`:

```toml
sqlx = { workspace = true, features = ["tls-rustls"] }
# error[rejected]: webpki-roots-0.26.11  license = "CDLA-Permissive-2.0"
#
# `ring` 0.17.14 is Apache-2.0 AND ISC and passes. The offender is the Mozilla
# CA root *bundle* — data, not code.
```

**Rejects.** An author wiring the Postgres adapter to a real database adds a TLS
backend, watches everything compile, and hands over a branch whose last gate
step is red for the licence on a certificate bundle — a rejection that reads as
a bug in `cargo deny` until you read the crate name. `tls-native-tls` is worse,
not better: it passes on Windows, where it resolves to `schannel`, and says
nothing about the Linux graph, which reaches `openssl-sys` instead. A green
local gate on one OS is then the whole of the evidence.

**Evidence.** `Cargo.toml:118 (webpki-roots-0.26.11)` ·
`Cargo.toml:121 (is the Mozilla CA root)` ·
`deny.toml:9 (version = 2)` · `deny.toml:2 (all-features = true)` ·
[cargo-deny licenses](https://embarkstudios.github.io/cargo-deny/checks/licenses/cfg.html) *(checked 2026-08-09, rustc 1.97.1)*

## RS-50-5. Do not add a dependency to run four assertions — model the shape with a stand-in and cite the real call site.

**Why.** Every dependency, dev-dependencies included — `[graph]` does not set
`exclude-dev` — is a graph `cargo deny` walks on every run and `cargo hack`
re-resolves once per feature subset. A stand-in costs the type checker nothing
and pins the same properties a signature can see.

**Do** — the shape, not the driver:

```rust
use std::rc::Rc;

/// Stand-in for a Workers `SqlStorage` handle. `Rc` is what makes it `!Send`;
/// `exec` is synchronous because the storage is co-located with the object.
struct SqlStorage(Rc<str>);

impl SqlStorage {
    fn exec(&self, sql: &str) -> Result<usize, Rc<str>> {
        if sql.is_empty() {
            return Err(Rc::clone(&self.0));
        }
        Ok(0)
    }
}

let storage = SqlStorage(Rc::from("[object SqlStorage]"));
// No await, no runtime, no driver in the graph.
assert_eq!(storage.exec("SELECT 1"), Ok(0));
assert!(storage.exec("").is_err());
```

**Not** — resolves, and every contributor pays it on every run of the gate:

```toml
# happenstance-cloudflare
worker = "0.6"             # drags wasm-bindgen, js-sys and web-sys
wasm-bindgen-test = "0.3"  # a dev-dependency for four assertions
```

**Rejects.** The author who adds `worker` to type-check against the real API
buys nothing a type checker can see: the four properties the skeleton exists to
pin — `!Send`, `!Sync`, a synchronous `exec`, a cursor that is not a snapshot —
are already modelled. What they spend is a `wasm-bindgen` licence graph on every
`cargo deny` and every `cargo hack` subset, for the whole workspace. Worse, the
crate stops building on the host, which removes the only `!Send` error type in
the tree from the native test run — and with it the only instrument ES-6 has.

**The trade this workspace later reversed, and what survives it.** Phase 9 added
`worker` to `happenstance-cloudflare` — not to run four assertions, but because
the crate stopped modelling a Durable Object and started binding one: the bodies
execute real SQL, and an instrument that models the runtime cannot be falsified
by the runtime. The rule is unchanged, and its *predictions* are worth reading
against what happened. The licence graph did widen, by 40 crates, and
`cargo deny check licenses` stayed green. The prediction that the crate would
stop building on the host was wrong: `wasm-bindgen` externs link on the host as
panicking stubs, so `cargo test -p happenstance-cloudflare` still runs the four
assertions in a contributor's inner loop. And the one cost the rule did not name
turned up anyway — `worker` depends on `async-trait`, which `deny.toml` bans, so
the swap needed a decision rather than a dependency line. The note it replaced is
kept in the manifest for the same reason this paragraph is kept here: the
reasoning for the state you are leaving is what a reviewer checks the change
against.

**Evidence.** `crates/happenstance-cloudflare/Cargo.toml:44 (was deliberately absent)` ·
`crates/happenstance-cloudflare/src/lib.rs:602 (exists to run four assertions)` ·
`crates/happenstance-cloudflare/src/sql_storage.rs:5 (Four properties are load-bearing)` ·
`xtask/Cargo.toml:17 (Deliberately absent)`
