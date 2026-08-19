---
title: "rust-event-sourcing-crate-landscape-and-sentiment — research for From Contract to Published Library"
kind: research
initiative: from-contract-to-published-library
summary: "Rust's event-sourcing crates are a long tail of storage-coupled, largely single-maintainer projects with no adopted storage-agnostic, DCB-conformant option; the closest thing to a competitor (UmaDB) shipped its Rust DCB port literally the day before this survey, and the pattern happenstance pursues (storage-agnostic DCB spanning a conventional database and a !Send edge runtime) already exists and works in other languages but not yet in Rust."
sources:
  - "https://crates.io/api/v1/crates/cqrs-es"
  - "https://crates.io/api/v1/crates/esrs"
  - "https://crates.io/api/v1/crates/eventually"
  - "https://crates.io/api/v1/crates/umadb-dcb"
  - "https://crates.io/api/v1/keywords/event-sourcing"
  - "https://umadb.io/"
  - "https://github.com/umadb-io/umadb"
  - "https://docs.rs/umadb-dcb/latest/umadb_dcb/"
  - "https://github.com/umadb-io/umadb/tree/main/umadb-dcb"
  - "https://dcb.events/resources/libraries/"
  - "https://github.com/disintegrate-es/disintegrate"
  - "https://doc.rust-cqrs.org/"
  - "https://github.com/thalo-rs/thalo"
  - "https://kgdev.me/posts/event-sourcing-cloudflare/"
  - "https://hackmd.io/@rust-lang-team/rJks8OdYa"
  - "https://smallcultfollowing.com/babysteps/blog/2023/02/01/async-trait-send-bounds-part-1-intro/"
---

# rust-event-sourcing-crate-landscape-and-sentiment

## Findings

- **The Rust event-sourcing crate landscape is a long tail with two active leaders, both storage-coupled, and no adopted storage-agnostic option.** `cqrs-es` (158,633 all-time downloads, 27,632 recent, last publish 2025-12-30) and `esrs` (122,482 all-time, 7,132 recent, last publish 2024-11-25, Prima.it-branded) are the two crates with real download volume, and both ship a fixed persistence story rather than a bare, adapter-implementable contract — `esrs` is explicitly built on `sqlx` with Postgres as its only shipped store.
- **The one prior Rust attempt at "storage-agnostic" event sourcing has been dead for six years.** `eventually-rs`, the crate search results themselves point to as the storage-agnostic example, peaked with an in-memory store and a Postgres backend, last published 2020-10-04, and now sits at 15,613 downloads with 70 recent — i.e. it stalled at exactly one real (non-memory) backend and was never picked up again.
- **A live, actively-published DCB-native competitor exists in Rust and is moving fast.** UmaDB (umadb-io/umadb, 82 GitHub stars, 9 forks) ships `umadb-dcb`, a Rust crate of "core interfaces and data structures for working with an event store that supports dynamic consistency boundaries" with sync and async trait pairs — conceptually adjacent to `happenstance-core`. Its most recent version (0.7.5) was published 2026-08-10, one day before this research initiative's intake, with downloads already at 3,247 total / 1,680 recent, showing real, current uptake.
- **UmaDB's value proposition is a different shape, not a substitute: a new dedicated store, not a port over stores you already run.** UmaDB ships a purpose-built Rust server (Copy-on-Write MVCC, non-blocking ACID) with client SDKs across Rust, Python, .NET, Elixir, Java, and PHP — you `cargo add` a client and run *their* store, rather than implementing a contract against a database you already operate or a runtime you're already deployed to (e.g. Cloudflare Durable Objects).
- **On the DCB specification's own implementations directory, Rust's listed entry is not UmaDB but `Disintegrate`** — described there as "DCB-inspired... slightly different approach." `Disintegrate` (123 stars, 10 forks, v4.0+, active, MIT) is architecturally storage-flexible in principle but has shipped exactly one backend crate, `disintegrate-postgres`, in its history — the same single-backend pattern as every other Rust entrant.
- **The storage-agnostic-with-pluggable-backends pattern happenstance pursues already exists and works in other languages, including the edge case.** PHP's `dcb-eventstore` ecosystem ships separate adapter packages for Doctrine DBAL and Illuminate (Laravel) spanning SQLite, MySQL/MariaDB, and PostgreSQL. Gleam's `factos` is described as "event store agnostic with pluggable backends" explicitly including Postgres, SQLite, *and Cloudflare Workers* — the same storage-plus-edge span happenstance's `!Send` port flavour exists to reach — but as of this survey there is no Rust equivalent with more than one live backend.
- **The `!Send` async-trait problem happenstance works around (ADR-0001) is a named, ecosystem-wide, still-unsolved Rust design gap, not an idiosyncratic worry.** The Rust language team held a dedicated design meeting on "Solving the Send bound problem" and Niko Matsakis wrote a multi-part public blog series on it; the standard workaround the community converged on is exactly the `trait_variant`-derived dual-trait shape happenstance already uses. None of the incumbent Rust ES crates surveyed (`cqrs-es`, `esrs`, `Disintegrate`) attempt this — they are built straightforwardly async/`tokio`-coupled, which is consistent with none of them targeting a `!Send` runtime.
- **Real first-person evidence confirms both the target audience and the difficulty of serving it.** A developer blog documenting hand-rolled event sourcing on Cloudflare Durable Objects in Rust reports genuine friction — `workers-rs` lagging the TypeScript SDK, WASM binary size ballooning to 2.2MB, sparse Durable Objects documentation requiring "half a dozen different blog posts and Discord threads" — and concludes "unless you specifically want Rust, TypeScript is probably the less painful path." The same author independently arrived at "the write path must never `await` mid-execution," a synchronous-command constraint structurally compatible with what a `!Send`, non-`async` port enables.
- **The community's own teaching material does not surface any of this as a live tradeoff.** `doc.rust-cqrs.org`, the standing introductory book for CQRS/event sourcing in Rust, introduces `cqrs-es` as *the* framework with no comparison, no mention of storage coupling, and no acknowledgment that multiple competing, largely-abandoned crates exist — so a newcomer following the canonical on-ramp is not told that "storage-agnostic, conformance-tested port" is even an axis crates differ on.
- **The crates.io keyword tag is a poor maturity signal on its own.** 311 crates carry the `event-sourcing` keyword, but the download and publish-date data above shows only a handful are meaningfully active — the raw count overstates how populated the space actually is with viable options.

## Evidence & citations

- `cqrs-es` downloads/version/publish date — https://crates.io/api/v1/crates/cqrs-es
- `esrs` downloads/version/publish date, Postgres/sqlx coupling — https://crates.io/api/v1/crates/esrs
- `eventually-rs` downloads/last-publish (2020-10-04, stalled) — https://crates.io/api/v1/crates/eventually
- `umadb-dcb` downloads/version/publish date (2026-08-10) — https://crates.io/api/v1/crates/umadb-dcb
- 311 crates under the `event-sourcing` keyword — https://crates.io/api/v1/keywords/event-sourcing
- UmaDB architecture (Copy-on-Write MVCC, multi-language client SDKs, purpose-built store) — https://umadb.io/
- UmaDB GitHub stars/forks (82/9), dual Apache-2.0/MIT — https://github.com/umadb-io/umadb
- `umadb-dcb` public API surface: `DcbEventStoreSync`/`DcbEventStoreAsync` trait pairs, docs coverage 63.11% — https://docs.rs/umadb-dcb/latest/umadb_dcb/
- `umadb-dcb` positioned for third-party DCB-compliant stores, not just UmaDB itself ("typically used as a dependency when building DCB-compliant event stores or working with UmaDB") — https://github.com/umadb-io/umadb/tree/main/umadb-dcb
- DCB.events implementations directory: per-language listing, Rust's entry is `Disintegrate` ("DCB-inspired... slightly different approach"), UmaDB filed separately as multi-language; PHP's Doctrine/Illuminate adapters across SQLite/MySQL/Postgres; Gleam's `factos` pluggable backends including Cloudflare Workers — https://dcb.events/resources/libraries/
- `Disintegrate` stars/forks/version (123/10/v4.0+), single shipped backend `disintegrate-postgres` — https://github.com/disintegrate-es/disintegrate
- `doc.rust-cqrs.org` introduces `cqrs-es` with no comparative or fragmentation framing — https://doc.rust-cqrs.org/
- Thalo: WASM/`sled`-based event sourcing runtime, explicitly "pre-release, API not stable" — a different niche (aggregate compute isolation), not a conformance-portable storage-agnostic port — https://github.com/thalo-rs/thalo
- First-person Cloudflare Durable Objects + Rust event sourcing account: `workers-rs` lag, WASM size, sparse docs, "unless you specifically want Rust, TypeScript is probably the less painful path," synchronous write-path design — https://kgdev.me/posts/event-sourcing-cloudflare/
- Rust language team design meeting, "Solving the Send bound problem in 2024" — https://hackmd.io/@rust-lang-team/rJks8OdYa
- Niko Matsakis, "Async trait send bounds, part 1: intro" — https://smallcultfollowing.com/babysteps/blog/2023/02/01/async-trait-send-bounds-part-1-intro/

## Implications for the idea

- **The audience segment the initiative targets — an application author who wants to `cargo add` a storage-agnostic, DCB-conformant contract in Rust — is currently unserved, not merely underserved.** Every crate with real adoption (`cqrs-es`, `esrs`, `Disintegrate`) makes the applicant pick a database first and a crate second; nobody today can pick the contract first. This is evidence the "who it is for" audience in the seed and intake brief is real, independent of whether happenstance's own design is right.
- **UmaDB is the fastest-moving thing in this space and is not a substitute, but it is not irrelevant either.** It solves DCB semantics via a new store you run rather than a port over stores you already operate — track its trajectory (it published a new version the day before this initiative's intake) because it will shape what outside observers compare a published happenstance against, and because its client-language spread shows where developer attention in the DCB space is currently going.
- **The one directly-comparable prior attempt at Rust storage-agnosticism (`eventually-rs`) died at exactly one non-memory backend.** That is a specific, on-point caution for the initiative's own emphasis (in the intake brief's desired outcome) on reaching adapters that "genuinely disagree" rather than stopping at a second skeleton — the historical failure mode in this exact space is stalling before a second real backend, not designing the wrong contract.
- **The `!Send`/edge audience segment is confirmed to exist and to be doing this work by hand today**, and the friction it reports is partly ecosystem-level (WASM tooling, Durable Objects documentation) rather than something a storage-agnostic contract crate alone resolves. The initiative's proof obligations should be read as addressing the "hand-rolling the event-sourcing layer" pain specifically, not as a claim that publishing removes every difficulty that first-person account describes.
- **The `!Send` design problem happenstance already committed to (ADR-0001) is validated as a genuine, unsolved, ecosystem-wide gap** by the Rust language team's own attention to it — this is external confirmation that the constraint is load-bearing for a real audience (anything wanting a `!Send` runtime), not an invented requirement, and that no incumbent Rust ES crate has taken it on.
- **The absence of any comparative framing in the community's own teaching material (`doc.rust-cqrs.org`) and the noisy 311-crate keyword tag mean prospective adopters currently have no way to discover that "storage-agnostic, conformance-tested" is even a choice.** Whatever the initiative ships, part of what changes the audience's situation is making that axis visible at all — a discoverability/legibility gap in the problem space, independent of any implementation choice.
- **Rust's storage-agnostic DCB space is not crowded, but it is not empty either, and it is moving.** Between UmaDB's fresh release cadence and `Disintegrate`'s active but single-backend project, "nobody is working on this in Rust" would be an inaccurate framing for the initiative to carry forward — "nobody in Rust has reached a genuinely storage-agnostic, adopted, conformance-proven result yet" is the framing the evidence actually supports.
