# 52 — wasm32, and target-conditional compilation

> **Load when:** `cargo check --target wasm32-unknown-unknown` failed · a test
> passes natively and panics on wasm32 · `dead_code` on one target only · an
> optional dependency lives in a per-target table · deciding what a green
> cross-target check proves · a conformance harness will not run
> **See also:** 51 (features) · 24 (the blocking bridge) · 20 (flavours) ·
> 41 (macros) · 80 (the gate)

---

## RS-52-1. On wasm32 the compiler is not the instrument: clocks, threads and `spawn_blocking` all type-check and then panic.

**Why.** `std` compiles for `wasm32-unknown-unknown` with large parts stubbed,
and the stubs panic rather than returning errors — `Instant::now()` is the
unsupported platform's `panic!("time not implemented on this platform")`, and
`std::thread::spawn` panics too. `tokio`'s `rt` feature *is* supported there, so
`spawn_blocking` compiles and then panics inside a pool built on `std::thread`.
No `cfg` reports any of this.

**Do** — take the clock as a parameter; the contract crate names no clock at all:

```rust
use happenstance_core::RecordedAt;

// The store stamps the time, so the type carries it and nothing here reads one.
fn stamp(now: RecordedAt) -> i64 { now.as_millis() }

assert_eq!(stamp(RecordedAt::from_millis(1_700_000_000_000)), 1_700_000_000_000);
```

**Not** — compiles for every target in the workspace, including the one it
cannot run on. The wasm32 *check* steps are compiles and see none of this; what
observes a panicking stub is `cargo xtask ci`'s `wasm32 run of the conformance
rules` step, wherever the runner is installed — which is every CI runner — and
only if some rule drives this path:

```rust
fn elapsed_ms() -> u128 {
    // On wasm32-unknown-unknown this line is a panic, not an `io::Error`:
    // there is no Result to handle and the module aborts.
    std::time::Instant::now().elapsed().as_millis()
}

assert!(elapsed_ms() < u128::MAX);
```

**Rejects.** `happenstance-cloudflare` has nothing `cfg`-gated in its non-test
code and documents that as a limit on its own evidence: a host build says the
crate is portable, not that a Durable Object adapter is. An author who reads
four green wasm32 steps as "it runs on Workers" ships an adapter that aborts the
module on its first recorded timestamp, found by a user in a Worker — where
`println!` writes nowhere, so nothing says why.

**Evidence.** `crates/happenstance-cloudflare/src/lib.rs:420 (The host build is a convenience rather than evidence)` ·
`crates/happenstance-core/src/identity.rs:154 (an adapter that has a clock)` ·
`xtask/src/main.rs:243 (name: "wasm32 build of the contract crate")` ·
[rustc — wasm32-unknown-unknown](https://doc.rust-lang.org/nightly/rustc/platform-support/wasm32-unknown-unknown.html) *(checked 2026-08-09, rustc 1.97.1)*

## RS-52-2. A feature is not target-scoped: an item behind a per-target optional dependency needs the target condition too.

**Why.** `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` scopes the
*dependency*. The feature that gates it is global, so `--all-features` sets
`feature = "proptest"` on wasm32 as well, where `dep:proptest` resolved to
nothing. A single `cfg(feature = "proptest")` then compiles the module against a
crate that is not in the graph.

**Do**

```toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
proptest = { workspace = true, optional = true }

[target.'cfg(target_arch = "wasm32")'.dev-dependencies]
wasm-bindgen-test.workspace = true
```

```rust
// Two conditions, because the feature crosses targets and the dependency does not.
#[cfg(all(feature = "proptest", not(target_arch = "wasm32")))]
pub mod strategies {
    pub fn any_tag() -> &'static str { "a" }
}
# fn main() {}
```

**Not** — one condition. On the target where the dependency resolved to nothing,
every path into the module is `error[E0433]`: *use of unresolved module or
unlinked crate*. Reproduced here by naming a crate that is genuinely absent from
this doctest's graph and present only in a `cfg(target_arch = "wasm32")`
dev-dependency table elsewhere in the workspace:

```rust,compile_fail,E0433
#[cfg(not(target_arch = "wasm32"))]
fn any_tag(t: wasm_bindgen_test::Thing) -> wasm_bindgen_test::Thing { t }
# fn main() {}
```

**Rejects.** The step that finds this is the wasm32 *feature powerset*, and it
sits behind a `cargo hack` probe. On a machine without the tool the mandatory
wasm32 check compiles one feature combination, prints nothing about the rest,
and the author is told the target is fine; the combination that fails is the one
`--all-features` produces, so it lands on CI or on the first adapter author who
enables the feature — in a crate they did not write, naming a crate they never
asked for.

**Evidence.** `crates/happenstance-testkit/Cargo.toml:115 (optional = true)` ·
`crates/happenstance-testkit/src/fixtures.rs:507 (feature is not target-scoped)` ·
`xtask/src/main.rs:937 (feature is not target-scoped)`

## RS-52-3. A `cfg` covers the probe *and* its caller, or the probe is dead code on the other target.

**Why.** A test-only helper whose caller is `cfg`-ed out is unreachable, and
`dead_code` fires. The gate compiles with `-D warnings`, so on that one target a
lint is an error — and it is the target no local `cargo test` ever reaches.

**Do** — one condition, written on both:

```rust
#![deny(dead_code)]

#[cfg(all(test, not(target_arch = "wasm32")))]
fn probe() -> bool { true }

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    #[test]
    fn the_probe_is_not_vacuous() { assert!(super::probe()); }
}

fn main() {}
```

**Not** — the diagnostic is a lint and carries **no error code**, so the fence
cannot name one; its first line is *"error: function `probe` is never used"*.
`#![deny(dead_code)]` stands in for the gate's ambient `-D warnings`:

```rust,compile_fail
#![deny(dead_code)]

#[cfg(not(target_arch = "wasm32"))]
fn probe() -> bool { true }

// One condition too many on the caller: the probe survives where nothing calls it.
#[cfg(all(test, not(target_arch = "wasm32")))]
fn caller() { assert!(probe()); }

fn main() {}
```

**Rejects.** The Cloudflare crate's `!Send` probe carries
`cfg(all(test, not(target_arch = "wasm32")))` for exactly this reason. Gate only
the caller and the wasm32 build of that crate — a mandatory, unskippable gate
step — goes red on a lint, in a crate whose bodies are all `todo!()`, with a
message about an unused function that says nothing about targets. The author
reproduces none of it locally, because `cargo test` never builds for wasm32.

**Evidence.** `crates/happenstance-cloudflare/tests/support/mod.rs:142 (is denied under)` ·
`crates/happenstance-cloudflare/src/lib.rs:601 (mod not_send_probe)` ·
`xtask/src/main.rs:316 (name: "wasm32 build of the Cloudflare adapter")`

## RS-52-4. The per-test attribute is the caller's, because `#[test]` cannot run on wasm32.

**Why.** Measured on 1.97.1: `#[test]` compiles for `wasm32-unknown-unknown` and
libtest links — `cargo test --no-run --target wasm32-unknown-unknown` emits a
`.wasm`. What is absent is a *runner*: cargo then tries to execute that file as a
native binary and reports *"could not execute process … (never executed)"*, so
`#[wasm_bindgen_test]` plus a configured cargo `runner` is the only way a test
actually runs there. A suite that emits its own attribute
hard-codes the set of targets it can run on. CF-23 states the obligation; this
is its spelling.

**Do** — the emitter is a parameter, captured as raw token trees because a
parsed `path` fragment cannot sit in callee position:

```rust
macro_rules! emit_blocking {
    ($($name:ident),* $(,)?) => { $( fn $name() {} )* };
}

macro_rules! for_each_rule {
    ($($callback:tt)+) => { $($callback)+! { append_is_atomic, positions_are_unique } };
}

for_each_rule!(emit_blocking);

fn main() {
    append_is_atomic();
    positions_are_unique();
}
```

**Not** — the attribute written into the suite's own expansion. It resolves in
the *caller's* scope, so in an adapter that does not carry that dev-dependency
on that target the expansion is `error[E0433]`. Substituting `#[tokio::test]`
does not fix it: that one compiles for wasm32 and then cannot run.

```rust,compile_fail,E0433
macro_rules! emit {
    ($($name:ident),*) => { $(
        #[::wasm_bindgen_test::wasm_bindgen_test]
        async fn $name() {}
    )* };
}

emit!(append_is_atomic);

fn main() {}
```

**Rejects.** With the attribute fixed inside the testkit, `happenstance-cloudflare`
would need `tokio` in its dev-dependencies to run a suite on a target tokio
cannot run on, and a `LocalSet` harness or a bare `block_on` would need a
testkit *release* to become usable at all. An adapter author's choice of harness
would then be held hostage by a version bump of the crate whose only job is to
grade them, and the failure would read as a missing dependency in their crate.

**Evidence.** `crates/happenstance-testkit/src/registry.rs:95 (captured as raw token trees)` ·
`crates/happenstance-testkit/src/registry.rs:282 (macro_rules! __emit_wasm {)` ·
`.github/workflows/ci.yml:147 (Install the wasm32 conformance runner)` ·
[SPECIFICATION CF-23](../../spec/SPECIFICATION.md)
