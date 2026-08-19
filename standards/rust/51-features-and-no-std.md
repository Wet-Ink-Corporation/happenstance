# 51 — Features, and the core/alloc/std ladder

> **Load when:** adding a `[features]` entry · a feature must not reach an
> optional dependency · the `no_std` build fails and the host build passes ·
> E0433 on `alloc` · choosing between `std::error::Error` and
> `core::error::Error` · writing `[package.metadata.docs.rs]`
> **See also:** 50 (dependency hygiene) · 52 (wasm32) · 70 (rustdoc) ·
> 62 (feature-gated examples) · 80 (the gate)

---

## RS-51-1. A feature adds. Spell the negative as `not(feature = "std")`, never as a feature named `no-std`.

**Why.** Cargo compiles a crate once per configuration with the *union* of every
feature any dependant in that build asked for. A feature that removes an item is
therefore switched by a crate you did not write, and `--all-features` — what the
gate's clippy, test and doc steps all run — turns it on unconditionally.

**Do** — name what is added, and write the negative with `not`:

```toml
[features]
default = ["std", "memory"]
std = ["thiserror/std", "bytes/std", "futures-core/std", "serde?/std"]
memory = ["std"]
```

```rust
// Both arms exist in every configuration; the union can only ever add.
#[cfg(feature = "std")]
fn source() -> &'static str { "std" }
#[cfg(not(feature = "std"))]
fn source() -> &'static str { "core" }

// This doctest's crate declares no feature named `std`, so the `not` arm is live.
assert_eq!(source(), "core");
```

**Not** — resolves and compiles. Whether the item exists is decided by whichever
dependant turned the feature on, and under `--all-features` that is always:

```toml
[features]
no-std = []   # subtractive: turning it ON deletes items
```

**Rejects.** `happenstance-sync` needs `std` and a Workers peer does not; one
build holds both, Cargo unifies, and the crate is compiled once with `no-std`
on. The dependant that never asked loses items it compiled against, and the
diagnostic surfaces in whichever crate happened to call one — none of which has
a feature line to blame. The gate's `--all-features` steps only ever see the
"on" configuration, so the combination that fails is the one nothing runs.

**Evidence.** `crates/happenstance-core/Cargo.toml:36 (serde?/std)` ·
`spec/SPECIFICATION.md:366 (Cargo features are additive)` ·
[Cargo Book — feature unification](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification) *(checked 2026-08-09, rustc 1.97.1)*

## RS-51-2. Reach an optional dependency with `dep:` and `?/`; a bare `dep/feature` in a default feature enables the dependency.

**Why.** `serde/std` enables the optional dependency `serde` as a side effect;
`serde?/std` adds the feature only if something else already enabled it. Naming
a dependency once as `dep:serde` also suppresses the implicit feature Cargo
would otherwise synthesise from its name — an implicit feature is public API a
consumer can turn on.

**Do**

```toml
std   = ["thiserror/std", "bytes/std", "futures-core/std", "serde?/std"]
serde = ["dep:serde", "dep:base64", "base64/alloc", "bytes/serde", "serde/alloc"]
```

**Not** — resolves, compiles, and every test passes. Measured on cargo 1.97.1:
dropping the `?` puts the optional dependency into the graph of everyone who
takes the defaults, and `cargo tree -e features` is the only place it shows:

```toml
default = ["std"]
std     = ["serde/std"]   # `std` now implies the serde dependency
serde   = ["bytes/serde"] # and this line no longer decides anything
```

**Rejects.** ADR-0003's guarantee is that `serde` is never in
`happenstance-core`'s default features — payload opacity, the wasm32 build's
size and the "no serialisation opinion" claim all rest on it. One missing `?`
breaks all three with nothing red: the crate builds, the conformance suite
passes, and the first symptom is a dependency in `cargo tree` that no manifest
line requested. The consumer who finds it is a `no_std` peer whose build has
newly acquired `serde/std`.

**Evidence.** `crates/happenstance-core/Cargo.toml:22 (optional = true)` ·
`crates/happenstance-core/Cargo.toml:49 (dep:base64)` ·
[ADR-0003](../../.kb/decisions/0003-opaque-payloads.md) ·
[Cargo Book — optional dependencies](https://doc.rust-lang.org/cargo/reference/features.html#optional-dependencies) *(checked 2026-08-09, rustc 1.97.1)*

## RS-51-3. Name every feature your code needs, even when today's lock file already supplies it.

**Why.** A feature that arrives because another crate enabled it for its own
optional dependency is a fact about the resolved graph, not about the API you
compiled against. The crate that withdraws it does so in a patch release, and
the build that breaks is one that changed nothing.

**Do**

```toml
# Stated, not inherited: this crate is no_std and its derives need `alloc`.
serde = ["dep:serde", "dep:base64", "base64/alloc", "bytes/serde", "serde/alloc"]
```

**Not** — compiles today, on this lock file, because `bytes` 1.12.1 happens to
enable `alloc` for its own optional `serde`. No compile and no test can observe
the difference, which is why the check is a manifest lint —
`cargo xtask lint-core-alloc-features` — rather than a conformance rule:

```toml
serde = ["dep:serde", "dep:base64", "bytes/serde"]
```

**Rejects.** Delete both tokens and every step of the gate stays green, so a
behavioural rule here would be decorative by construction. The failure arrives
later as `String::deserialize` failing to resolve *inside* `happenstance-core`,
in a downstream `no_std` replication peer's CI, on the day `bytes` ships a patch
release — with no commit of this workspace's behind it and no manifest line
anywhere that changed.

**Evidence.** `crates/happenstance-core/Cargo.toml:39 (stated rather than inherited)` ·
`xtask/src/main.rs:612 (names serde/alloc and base64/alloc)` ·
`xtask/src/lints.rs:378 (fn core_alloc_features)` ·
[ADR-0016](../../.kb/decisions/0016-the-wire-format.md)

## RS-51-4. `extern crate alloc;` is the only way onto the ladder, and `core::error::Error` is the only bound worth writing.

**Why.** `core` is in the extern prelude and `alloc` is not — no edition changed
that — so an `alloc`-using crate needs the `extern crate` line in its root.
`std::error::Error` has been a re-export of `core::error::Error` since 1.81, so
the `core` path names the same trait and costs a `std` consumer nothing.

**Do**

```rust
extern crate alloc;

use alloc::string::String;

#[derive(Debug)]
struct DecodeError(String);
# impl core::fmt::Display for DecodeError {
#     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
#         f.write_str(&self.0)
#     }
# }
impl core::error::Error for DecodeError {}

// The port's bound is `core::error::Error + 'static`. A std caller's bound is
// the same trait, so one impl satisfies both — no feature gate, no second impl.
fn assert_std_error<E: std::error::Error>() {}

fn main() {
    assert_std_error::<DecodeError>();
    let _ = DecodeError(String::from("bad tag"));
}
```

**Not** — `error[E0433]`, whose `help` line is the whole fix: *"add `extern crate
alloc` to use the `alloc` crate"*.

```rust,compile_fail,E0433
use alloc::string::String;

fn main() {
    let _ = String::new();
}
```

**Rejects.** `ConditionViolated` implements `core::error::Error` by hand. Write
it as `#[cfg(feature = "std")] impl std::error::Error` instead and the type stops
satisfying `EventStore::Error`'s own bound in exactly the configuration the
contract crate exists to support, taking every adapter's `source` chain with it.
The host build, `cargo test --workspace --all-features` and clippy are all
green; the mandatory `--no-default-features` doc build of `happenstance-core` is
the only step in the gate that sees it.

**Evidence.** `crates/happenstance-core/src/lib.rs:115 (extern crate alloc)` ·
`crates/happenstance-core/src/error.rs:184 (impl core::error::Error for ConditionViolated)` ·
`crates/happenstance-core/src/store.rs:101 (core::error::Error + 'static)` ·
[core::error::Error](https://doc.rust-lang.org/core/error/trait.Error.html) *(checked 2026-08-09, rustc 1.97.1)*

## RS-51-5. Keep `doc_cfg` behind `cfg_attr(docsrs, …)`, and declare the docs.rs configuration in the manifest.

**Why.** docs.rs builds *after* publication, on nightly, and passes `--cfg
docsrs` to the final rustdoc invocation itself — the manifest's `rustdoc-args`
line has been redundant since February 2024, and is kept only because it states
the dependency. A crates.io release can be yanked and never edited, so a
`doc_cfg` spelling that fails there is the one class of breakage with no fix.

**Do** — `doc_cfg` is still unstable in August 2026, so the feature gate and
every attribute that needs it sit behind the same `cfg`:

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub fn wire() {}

fn main() { wire(); }
```

```toml
[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

**Not** — compiles everywhere `docsrs` is unset, which is everywhere except
docs.rs. `doc_auto_cfg` was removed in 1.92 and merged into `doc_cfg`, so on
current nightly this fails with *"feature has been removed"* — and the nightly
`RUSTDOCFLAGS="--cfg docsrs"` rustdoc build in `cargo xtask ci` is the only
thing in the tree that compiles the line at all:

```rust
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

fn main() {}
```

**Rejects.** The breakage is invisible to every stable step and to every reader,
because the attribute is inert until docs.rs sets the cfg. It surfaces as a
failed build on the crate's own documentation page, immediately after the one
event in a crate's life that cannot be undone: a yank removes the version from
the resolver and leaves the page exactly as it is, so the first impression the
crate makes is a build log, and the fix ships as the *next* version.

**Evidence.** `crates/happenstance-core/Cargo.toml:99 (package.metadata.docs.rs)` ·
`crates/happenstance-core/src/lib.rs:113 (feature(doc_cfg))` ·
`xtask/src/main.rs:805 (is a cfg nobody sets except docs.rs)` ·
[docs.rs metadata](https://docs.rs/about/metadata) *(checked 2026-08-09, rustc 1.97.1)*
