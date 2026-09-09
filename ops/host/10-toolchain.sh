#!/usr/bin/env bash
#
# The user half of provisioning: the Rust toolchain and the tools `cargo xtask
# ci` probes for. No sudo, and nothing here writes outside $HOME.
#
# Run it AFTER `00-system.sh` and after the checkout exists, because two of the
# versions below are read out of the checkout rather than written down here:
# the toolchain from `rust-toolchain.toml`, and `wasm-bindgen-cli` from
# `Cargo.lock`. A second copy of either is a second thing to update and one that
# goes stale.
#
#   usage: ./10-toolchain.sh [checkout-dir]     (default: ~/src/happenstance)

set -euo pipefail

CHECKOUT="${1:-$HOME/src/happenstance}"

note() { printf '10-toolchain: %s\n' "$*"; }
die()  { printf '10-toolchain: %s\n' "$*" >&2; exit 1; }

[ -f "$CHECKOUT/rust-toolchain.toml" ] || die "no checkout at $CHECKOUT (expected rust-toolchain.toml)"

# --- rustup -----------------------------------------------------------------
# `--default-toolchain none` on purpose: `rust-toolchain.toml` pins the channel,
# the two components AND the wasm32 target, so the first `cargo` run inside the
# checkout installs exactly the right set. Naming a channel here would be a
# second copy of the pin, and copies of a pin drift.
if ! command -v rustup >/dev/null 2>&1; then
  note "installing rustup (no default toolchain -- rust-toolchain.toml decides)"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --no-modify-path --default-toolchain none --profile minimal
fi
export PATH="$HOME/.cargo/bin:$PATH"

# `--no-modify-path` above keeps rustup out of the shell profile, so put it there
# once, deliberately, and BEFORE the interactivity guard Ubuntu's .bashrc opens
# with -- otherwise `ssh host cargo --version` finds nothing on a host where the
# toolchain is installed. preflight.sh and bench.sh also prepend it themselves;
# this line is for a human who logs in.
if ! grep -q 'happenstance: rustup on PATH' "$HOME/.bashrc" 2>/dev/null; then
  {
    echo '# happenstance: rustup on PATH. Deliberately at the TOP, because'
    echo '# Ubuntu .bashrc returns early for a non-interactive shell and'
    echo '# `ssh host cargo --version` is exactly that.'
    echo 'case ":$PATH:" in *":$HOME/.cargo/bin:"*) ;; *) PATH="$HOME/.cargo/bin:$PATH";; esac'
    echo
    cat "$HOME/.bashrc" 2>/dev/null || true
  } > "$HOME/.bashrc.new"
  mv "$HOME/.bashrc.new" "$HOME/.bashrc"
  note "prepended ~/.cargo/bin to PATH in ~/.bashrc"
fi

note "materialising the pinned toolchain from $CHECKOUT/rust-toolchain.toml"
( cd "$CHECKOUT" && cargo --version && rustc --version )

# --- nightly ----------------------------------------------------------------
# Explicit, because the `--cfg docsrs` rustdoc build is an OPTIONAL step in
# `cargo xtask ci` guarded by a `cargo +nightly --version` probe: a missing
# nightly does not fail the gate, it prints `skipped:`. On this host a skipped
# step is a hole.
#
# `--target wasm32-unknown-unknown` is load-bearing and ci.yml says so:
# rust-toolchain.toml's `targets` applies to the PINNED channel only, so nightly
# gets the target or the wasm32 docsrs build cannot run.
note "installing nightly (+wasm32) for the docs.rs configuration step"
rustup toolchain install nightly --profile minimal --target wasm32-unknown-unknown

# --- the tools the gate probes for -----------------------------------------
note "installing cargo-hack and cargo-deny (the two OPTIONAL gate steps)"
cargo install --locked cargo-hack cargo-deny

# Not gate tools. cargo-mutants because the 0.2.0 certification review calls it
# "the single highest-yield unrun experiment in the tree, and it costs one
# command"; cargo-semver-checks because the semver job has a registry baseline
# now that five crates are published.
note "installing cargo-mutants and cargo-semver-checks"
cargo install --locked cargo-mutants cargo-semver-checks

# --- wasm-bindgen-cli, at the version Cargo.lock resolves -------------------
# The CLI must match the crate's schema exactly; a `cargo update` invalidates an
# installed one. This is `.github/workflows/ci.yml`'s own derivation, verbatim,
# so the host and CI cannot disagree about which runner to use.
WB_VERSION="$(cd "$CHECKOUT" && cargo metadata --locked --format-version 1 \
  | jq -r '.packages[] | select(.name == "wasm-bindgen") | .version')"
[ -n "$WB_VERSION" ] || die "could not resolve wasm-bindgen version from Cargo.lock"
note "installing wasm-bindgen-cli@$WB_VERSION (from Cargo.lock, as CI does)"
cargo install --locked "wasm-bindgen-cli@$WB_VERSION"

# --- the Postgres image the fixture pins ------------------------------------
# Pre-pulled, not composed. `.github/workflows/ci.yml` declines a `services:`
# block on purpose -- "the image, its tag and its max_connections are the
# fixture's" (crates/happenstance-postgres/tests/support/mod.rs) -- and a compose
# file here would fork that decision. testcontainers starts its own.
if command -v docker >/dev/null 2>&1; then
  note "pre-pulling postgres:17.10 for the testcontainers fixture"
  docker pull -q postgres:17.10
fi

# --- secrets ----------------------------------------------------------------
# Not exported from .bashrc. NEON_CONNECTION must be sourced deliberately: the
# certification review records the 512 MB Neon project exhausting at run 8, so a
# shell that always carries it turns an absent-minded `cargo test --ignored` into
# a burnt project.
mkdir -p "$HOME/.config/happenstance"
if [ ! -e "$HOME/.config/happenstance/env" ]; then
  cat > "$HOME/.config/happenstance/env" <<'ENVEOF'
# Sourced deliberately, never from .bashrc:
#   set -a; . ~/.config/happenstance/env; set +a
# NEON_CONNECTION=postgresql://...
ENVEOF
  chmod 600 "$HOME/.config/happenstance/env"
  note "created ~/.config/happenstance/env (0600) -- add NEON_CONNECTION to it"
fi

note "done. Next: ops/host/preflight.sh --report"
