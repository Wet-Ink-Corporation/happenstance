#!/usr/bin/env bash
#
# Re-derives everything under `results/` from a clean checkout.
#
# NF-001: a figure a second person cannot reproduce is a claim with a table
# attached. This script is the whole of what produced them; the README states the
# machine, the toolchain and the build profile, and every figure is printed
# beside the settings it was produced on.
#
# It is NOT a gate step and must never become one. CF-34: a measurement harness
# is not part of the conformance bar, and a measurement that can turn a merge red
# teaches people to re-run until green. Nothing in `.redkiln/config.yaml` or
# `cargo xtask ci` invokes this, and the crate carries an empty `[workspace]`
# table so cargo cannot see it from the root.
#
# Wall-clock budget on the machine in the README: under two minutes, almost all
# of it the first build. NF-003 requires it to terminate unattended, which it
# does — there is no I/O, no network, no clock and no timeout anywhere in it. The
# one store here that suspends inside `append` signals its waker *before*
# returning `Pending`, so a rule waiting on it is slow rather than hung; there is
# deliberately no watchdog, for CF-33's reason.

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$here"

mkdir -p results/raw

echo "==> the conditions this run was produced under"
{
  echo "date:     $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
  echo "commit:   $(git -C "$here" rev-parse HEAD)"
  echo "rustc:    $(rustc -vV | tr '\n' ' ')"
  echo "cargo:    $(cargo -V)"
  echo "rustflags from .cargo/config.toml (inherited from the repository root,"
  echo "  because cargo config discovery walks UP across workspace boundaries):"
  echo "          $(grep -A1 '^\[build\]' ../../.cargo/config.toml | tail -1)"
} | tee results/raw/conditions.txt

echo
echo "==> conformance first: the controls, mounted the way an adapter mounts the suite"
echo "    An instrument that certifies a wrong store because it never ran the rules"
echo "    produces exactly the output this experiment is looking for, so the"
echo "    machinery is checked by libtest before it is trusted to report."
cargo test --test controls_are_conformant 2>&1 | tee results/raw/controls.txt

echo
echo "==> each defect is real: driven through the port, with no rule in the path"
echo "    A strawman passes the suite for the same reason a correct store does."
cargo test --test defect_is_real -- --nocapture 2>&1 | tee results/raw/defects.txt

echo
echo "==> the census: every registered event-store rule against every subject"
cargo test --release --test census -- --nocapture --test-threads=1 2>&1 \
  | tee results/raw/census.txt

echo
echo "The answer is the ANSWER: line in results/raw/census.txt. The tables in"
echo "results/*.md are written by hand from these files, because a table nobody"
echo "read is a table nobody checked."
