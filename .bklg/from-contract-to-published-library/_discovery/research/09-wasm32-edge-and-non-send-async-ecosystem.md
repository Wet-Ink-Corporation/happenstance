---
title: "wasm32-edge-and-non-send-async-ecosystem — research for From Contract to Published Library"
kind: research
initiative: from-contract-to-published-library
summary: "The !Send edge-Rust pain the contract's two-flavour design targets is real, live, and unresolved upstream — but the niche (event sourcing reaching wasm) has a graveyard, and the target itself is not one thing."
sources:
  - https://developers.cloudflare.com/workers/languages/rust/
  - https://github.com/cloudflare/workers-rs/issues/485
  - https://smallcultfollowing.com/babysteps/blog/2023/02/01/async-trait-send-bounds-part-1-intro/
  - https://crates.io/crates/trait-variant
  - https://internals.rust-lang.org/t/trait-variant-crate-now-available-for-afit-and-rpitit/20050
  - https://rymnc.com/posts/eager-to-remove-async-trait/
  - https://github.com/cloudflare/workers-rs/issues/967
  - https://developers.cloudflare.com/durable-objects/api/sqlite-storage-api/
  - https://blog.cloudflare.com/sqlite-in-durable-objects/
  - https://github.com/thalo-rs/thalo
  - https://lib.rs/crates/thalo
  - https://doc.rust-lang.org/beta/rustc/platform-support/wasm32-wasip3.html
  - https://doc.rust-lang.org/nightly/nightly-rustc/rustc_target/spec/targets/wasm32_unknown_unknown/index.html
  - https://github.com/rustwasm/wasm-bindgen/issues/2833
---
# wasm32-edge-and-non-send-async-ecosystem

## Findings

- **Cloudflare's own Rust target structurally forbids the mainstream async runtime, not just a library choice.** Workers-rs runs the whole handler as a single JS `Promise` via `spawn_local` on the JS event loop; Tokio and async-std are unusable on this target regardless of what any individual crate does. This is a property of the platform, not a workaround a library can route around. (developers.cloudflare.com/workers/languages/rust/)

- **The `!Send` boundary on this target is not hypothetical — it breaks real, non-trivial integration attempts today.** `workers-rs` issue #485 documents a user whose Axum integration compiled for the trivial example but failed once R2 storage entered the picture, because `JsFuture` wraps `Rc<RefCell<...>>` and cannot be `Send`. The user's own words: the maintainers "recently added an example with axum" but "any non-trivial example" fails, and they ask the maintainer directly "how do you imagine this should work?" — i.e. the platform's own maintainers had not resolved this for a common case. (github.com/cloudflare/workers-rs/issues/485)

- **The ecosystem has a blessed, named pattern for exactly this problem, and it is a minority path.** The `trait-variant` crate, maintained under the `rust-lang` org, generates a `Send`-bounded twin of a base async trait via `#[trait_variant::make(SendTrait: Send)]` — this is the same two-definitions-one-derived shape the contract in this repository uses. But ecosystem guidance as of the post-AFIT (native async-fn-in-traits, stabilized Rust 1.75, Dec 2023) migration wave is to *keep* `async_trait` — the `Send`-injecting macro — whenever a trait needs `dyn Trait`, is public API "where callers might need object safety," or is used with `tokio::spawn`. That covers most public Rust library traits. The `!Send`-safe path is the deliberate exception, not the default anyone reaches for without a wasm32 target already in front of them. (crates.io/crates/trait-variant; internals.rust-lang.org/t/trait-variant-crate-now-available-for-afit-and-rpitit/20050; rymnc.com/posts/eager-to-remove-async-trait)

- **Cloudflare's own platform has not reached Rust/JS parity on the exact storage shape this contract cares about.** Durable Objects gained a SQLite-backed storage layer with a **synchronous** KV API in JavaScript (GA in 2026, feature parity with the legacy KV backend, point-in-time recovery to any point in the last 30 days) — but the Rust `worker` crate exposes only the asynchronous wrapper around it. A feature request to expose the synchronous API to Rust (issue #967) sits open on a repository with 3.6k stars and 126 open issues, filed in 2024 and still unresolved as of this research. Rust developers building storage on this specific edge target do not have the same primitive JS developers have. (github.com/cloudflare/workers-rs/issues/967; developers.cloudflare.com/durable-objects/api/sqlite-storage-api/; blog.cloudflare.com/sqlite-in-durable-objects/)

- **Prior art for "event sourcing that reaches WebAssembly" exists, and it did not survive as a going concern.** Thalo — an event-sourcing runtime combining `wasmtime` (to compile aggregates to wasm) with `sled` as an embedded event store — is now explicitly unmaintained; its own listing directs new projects to successor projects (SierraDB, kameo_es) instead. The niche keeps getting reattempted rather than solved once by a lasting project. (github.com/thalo-rs/thalo; lib.rs/crates/thalo)

- **"wasm32" is not one target, and the platform this contract targets is the narrowest of the three.** `wasm32-unknown-unknown` (bare-bones, no syscalls, the ABI wasm-bindgen and Workers use) is a different runtime model from `wasm32-wasip1`/`wasip2`/`wasip3` (POSIX-ish WASI, with `wasip3` only now bringing *native* async support) and from `wasm32-unknown-emscripten`. They are not interoperable. Evidence gathered against "Rust on wasm" broadly — including improving native-async story on WASI — does not automatically transfer to the Durable-Object / JS-event-loop model this project's own constraint is written against. (doc.rust-lang.org platform-support pages for wasip3 and unknown-unknown)

- **Adjacent local-first/edge Rust data projects exist and are active, but none found overlaps this niche directly.** PGlite (embeddable Postgres in WASM, reactive live queries) and VelesDB (local-first vector/graph/columnar store, ~6MB binary) show live demand for "runs entirely at the edge, in Rust or Rust-adjacent tooling," but neither is a DCB-style, storage-agnostic, multi-adapter event-sourcing contract — no direct occupant of this specific niche turned up in this research pass (a negative result, held cautiously rather than as proof of a clear field).

## Evidence & citations

| Claim | Source |
| --- | --- |
| Workers-rs forces the whole handler through `spawn_local` on the JS event loop; Tokio/async-std unusable | https://developers.cloudflare.com/workers/languages/rust/ |
| Real Axum + R2 integration broke on `!Send` `JsFuture`; maintainers asked directly how it should work | https://github.com/cloudflare/workers-rs/issues/485 |
| `wasm_bindgen_futures` futures are fundamentally not `Send` | https://github.com/rustwasm/wasm-bindgen/issues/2833 |
| Async trait `Send`-bound problem explained generally, including why `Box<dyn Future + Send>` "loses key flexibility" and fails no-std | https://smallcultfollowing.com/babysteps/blog/2023/02/01/async-trait-send-bounds-part-1-intro/ |
| `trait-variant` crate generates a `Send` twin trait; rust-lang-org maintained | https://crates.io/crates/trait-variant ; https://internals.rust-lang.org/t/trait-variant-crate-now-available-for-afit-and-rpitit/20050 |
| Post-AFIT ecosystem guidance: keep `async_trait` (Send-injecting) unless you have a specific reason not to | https://rymnc.com/posts/eager-to-remove-async-trait/ |
| Durable Objects SQLite storage: synchronous KV API is GA in JS, with point-in-time recovery | https://developers.cloudflare.com/durable-objects/api/sqlite-storage-api/ ; https://blog.cloudflare.com/sqlite-in-durable-objects/ |
| Rust `worker` crate lacks the synchronous KV API JS has; open feature request since 2024, repo has 3.6k stars / 126 open issues | https://github.com/cloudflare/workers-rs/issues/967 |
| Thalo (wasmtime + sled event-sourcing runtime) is unmaintained; successors named | https://github.com/thalo-rs/thalo ; https://lib.rs/crates/thalo |
| `wasm32-unknown-unknown` vs `wasip1/2/3` are distinct, non-interoperable ABIs/runtime models; `wasip3` only now gets native async | https://doc.rust-lang.org/beta/rustc/platform-support/wasm32-wasip3.html ; https://doc.rust-lang.org/nightly/nightly-rustc/rustc_target/spec/targets/wasm32_unknown_unknown/index.html |

## Implications for the idea

- **The pain the seed cites is not hypothetical — there is a public trail of real users hitting it on this exact platform.** Issue #485 is direct, dated evidence that "application author on the edge target has no good storage-agnostic option" is a live condition, not a designed-in worst case. That strengthens the seed's framing of who this is for, and suggests the audience can be found where these issues are filed and discussed, not just inferred.

- **The design choice this contract already made (two trait flavours, one derived) matches a pattern the Rust async working group itself endorses — but the ecosystem's default gravity still pulls the other way.** Most public async trait authors are being told to *keep* the Send-injecting macro unless they specifically need the exception. That means the audience for the "does not drop `!Send`" property is real but is not the median Rust library consumer's starting expectation; anyone evaluating this library against "how does async_trait handle it" will be comparing against the common case, not the edge case this project was built for. That is a fact about how the value has to be explained, not a fact to design around.

- **The platform this scenario is written against has not finished catching up to itself.** Cloudflare shipped a synchronous, transactional SQLite storage primitive to JavaScript Durable Objects and has not yet shipped the Rust equivalent. Any claim that "the contract has met a database" by way of a Durable Object will be meeting a target whose own host runtime is mid-migration for Rust specifically — which is a fact about the target's maturity independent of anything this project does, and worth naming rather than absorbing silently as if it were this project's gap.

- **The niche has already produced at least one serious, now-abandoned attempt.** Thalo tried "event sourcing reaches WebAssembly" with a different shape (compile aggregates to wasm, embedded `sled` store) and did not last as a maintained project. That is evidence the recurring need is real (people keep trying), and separately, a caution that whatever makes an attempt in this space *durable* — not just correct on day one — is itself part of what remains unproven, alongside the six things the seed already names as unfinished.

- **"wasm32" as a description of the constraint is broader than the concrete target the scenario cares about.** The seed's own language ("run inside a Cloudflare Durable Object on wasm32") points at the narrowest, least POSIX-like of the three wasm32 ABIs — the one with no native async story of its own, unlike `wasip3`. Evidence or measurement gathered under a different wasm32 flavor (e.g. WASI's improving async) would not transfer, and framing the constraint's scope precisely (this specific runtime model, not "wasm" as a category) matters for anyone later deciding what "the edge case" was actually validated against.
