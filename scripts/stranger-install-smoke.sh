#!/usr/bin/env bash
#
# DR-8 — the stranger-install smoke.
#
# Builds a scratch project OUTSIDE this workspace, with no path dependency and no
# `[patch]`, depending on the published `happenstance` and `happenstance-sqlite`
# by version alone, and completes a write-then-read cycle against them.
#
# ## What it is for, and what the gate cannot do instead
#
# Every check in `cargo xtask ci` runs inside this workspace, where the crates
# resolve by path. A path dependency ignores the `include` list, ignores whether
# a file was left out of the `.crate`, ignores a feature that only unifies
# because a sibling crate turned it on, and ignores a `version` requirement that
# names something the registry does not have. `cargo package --list` catches some
# of that and is already a gate step; it cannot catch a crate that packages
# correctly and then fails to *compile* for somebody who has only the registry.
#
# So this is deliberately not a gate step: it needs the release to exist. It runs
# AFTER `cargo publish`, before the tag is announced, and its failure mode is a
# yank rather than a red build.
#
# ## Running it
#
#   scripts/stranger-install-smoke.sh            # against the published version
#   VERSION=0.3.0 scripts/stranger-install-smoke.sh
#
# It needs network access and a writable temp directory. It creates nothing
# inside this repository, which is the point: a scratch project that inherited
# this workspace's `Cargo.toml`, its `.cargo/config.toml` or its lockfile would be
# testing the thing it exists to avoid.

set -euo pipefail

VERSION="${VERSION:-0.3.0}"
WORKDIR="$(mktemp -d)"
trap 'rm -rf "$WORKDIR"' EXIT

echo "stranger-install smoke: happenstance ${VERSION}, in ${WORKDIR}"

# `--offline` is deliberately NOT passed anywhere below. The whole proposition is
# that a stranger resolves these from the registry.
cd "$WORKDIR"
cargo init --name stranger --bin --vcs none >/dev/null 2>&1

cat > Cargo.toml <<TOML
[package]
name = "stranger"
version = "0.0.0"
edition = "2024"

# By version, never by path, and with no [patch] section. A path dependency here
# would resolve the working tree and prove nothing about the .crate that shipped.
[dependencies]
happenstance = { version = "${VERSION}", features = ["json"] }
happenstance-core = { version = "${VERSION}", features = ["std"] }
happenstance-sqlite = { version = "${VERSION}", features = ["event-store"] }
futures-util = "0.3"
tokio = { version = "1", features = ["macros", "rt"] }
TOML

# An empty [workspace] table so this project cannot be absorbed into any
# workspace above it in the filesystem — which on some temp layouts is a real
# possibility and would silently reintroduce the path dependencies.
printf '\n[workspace]\n' >> Cargo.toml

cat > src/main.rs <<'RUST'
//! The smallest thing that is still a write-then-read cycle.
//!
//! It appends one event through the published typed layer and reads it back
//! through the published SQLite adapter. Anything less — constructing a value
//! and dropping it — would compile against a crate whose store never linked.

use happenstance_core::{EventStore, Query, ReadOptions};
use happenstance_sqlite::event_store::SqliteEventStore;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = SqliteEventStore::open_in_memory()?;

    let event = happenstance_core::Event::new("Stranger.Arrived", &b"{}"[..])?;
    let position = store.append(&[event], None).await?;
    println!("appended at {position:?}");

    let mut seen = 0usize;
    {
        use futures_util::StreamExt as _;
        // `query` is bound rather than inlined: `read` borrows it and returns
        // the stream at the top level, so an inline `&Query::all()` is a
        // temporary freed at the end of the statement — `error[E0716]`. That is
        // ES-3's signature doing exactly what it promises, and it is the first
        // thing a stranger meets.
        let query = Query::all();
        let stream = store.read(&query, ReadOptions::new());
        let mut stream = Box::pin(stream);
        while let Some(item) = stream.next().await {
            item?;
            seen += 1;
        }
    }

    assert_eq!(seen, 1, "wrote one event and read back {seen}");
    println!("stranger-install smoke: OK");
    Ok(())
}
RUST

echo "--- resolving from the registry ---"
cargo build --quiet

echo "--- the write-then-read cycle ---"
cargo run --quiet

echo
echo "stranger-install smoke PASSED for ${VERSION}"
