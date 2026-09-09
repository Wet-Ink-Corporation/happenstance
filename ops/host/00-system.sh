#!/usr/bin/env bash
#
# The root half of provisioning the measurement host: packages, the things that
# would otherwise suspend the machine mid-run, and the things that would
# otherwise wake up and compete with it.
#
# Idempotent. `--restore` undoes the masking and removes the unit; it does not
# uninstall packages, because an installed package is not a condition anything
# measures.
#
# The boundary this script sits on: it only WRITES. `preflight.sh` only READS.
# A script that both changes the machine and judges it can always make its own
# check pass — which is the same argument `benchmarks/README.md` makes for
# `tests/instruments_work.rs` being a separate binary from the instruments.

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LIBDIR=/usr/local/lib/happenstance-bench-host

die()  { printf '00-system: %s\n' "$*" >&2; exit 1; }
note() { printf '00-system: %s\n' "$*"; }

[[ $EUID -eq 0 ]] || die "must run as root: sudo $0"

PACKAGES=(
  build-essential pkg-config
  # `lbug`'s driver links against OpenSSL, and this is the host that could run
  # the LadybugDB conformance suite — the one adapter in the workspace whose
  # conformance has never been observed by a second machine.
  libssl-dev
  cmake clang lld
  git curl jq unzip ca-certificates
  postgresql-client
  # `wasm-bindgen-test-runner` executes the wasm32 conformance rules under Node,
  # and without it `cargo xtask ci` fails at "wasm32 run of the conformance
  # rules" with `failed to find or execute Node.js`. GitHub's ubuntu-latest
  # ships Node, so CI never needed to say this out loud and a fresh host does.
  #
  # Worth noting what happened when it was missing: the gate went RED, not
  # green-with-a-skip. That is the property the 0.2.0 certification review asks
  # for in its fourth blocking condition -- 93 wasm32 rules must not be
  # silenceable with the gate still green.
  #
  # NOT the distro `nodejs` package. Ubuntu 24.04 ships v18.19.1, and
  # wasm-bindgen 0.2.126's output crashes V8 on it -- a raw
  # "==== JS stack trace ====" dump out of node, not a test failure. GitHub's
  # ubuntu-latest ships Node 20+, which is why CI never met this. Installed from
  # the official tarball below instead, at a pinned version: a machine whose
  # value is that nothing changes under it should not carry an auto-updating
  # third-party apt repository.
  # sensors + throttle counters for preflight; perf for anything that follows.
  lm-sensors sysstat linux-tools-common "linux-tools-$(uname -r)"
  util-linux
)

# Masked, not merely disabled. `apt-get install` re-enables a *disabled* timer
# as a side effect of a later unrelated install; a masked unit stays masked.
# This is the one interference class that fires on a schedule, which means it
# lands entirely inside whichever arm happened to be running — the exact shape
# `benchmarks/src/paired.rs` interleaves arms to defend against, and which it
# can only report rather than remove.
NOISY_UNITS=(
  apt-daily.timer apt-daily-upgrade.timer
  unattended-upgrades.service
  man-db.timer motd-news.timer motd-news.service
  fstrim.timer
  fwupd-refresh.timer
  apport.service
  ModemManager.service
  # Present on this host and inert on bare metal; it wakes, finds no hypervisor
  # and goes away again, which is a wake-up nonetheless.
  open-vm-tools.service
)

SLEEP_TARGETS=(sleep.target suspend.target hibernate.target hybrid-sleep.target)

# Pinned, not "latest". See the PACKAGES comment about the distro nodejs.
NODE_VERSION=v22.20.0
NODE_MAJOR=22

apply() {
  note "installing packages"
  export DEBIAN_FRONTEND=noninteractive
  apt-get update -qq
  apt-get install -y -qq "${PACKAGES[@]}"

  # --- Node, for the wasm32 conformance runner ---------------------------------
  # Pinned tarball into /usr/local. `node --version` is checked rather than
  # trusted so a partial extraction is not mistaken for an install.
  if ! /usr/local/bin/node --version 2>/dev/null | grep -q "^v${NODE_MAJOR}[.]"; then
    note "installing Node $NODE_VERSION (distro ships v18, which V8-crashes on wasm-bindgen output)"
    tmp="$(mktemp -d)"
    url="https://nodejs.org/dist/${NODE_VERSION}/node-${NODE_VERSION}-linux-x64.tar.xz"
    curl -fsSL -o "$tmp/node.tar.xz" "$url"
    tar -xJf "$tmp/node.tar.xz" -C "$tmp"
    cp -a "$tmp/node-${NODE_VERSION}-linux-x64/." /usr/local/
    rm -rf "$tmp"
  fi
  note "node $(/usr/local/bin/node --version)"

  note "masking sleep — a laptop that suspends at minute 12 of a 40-minute run loses the run"
  systemctl mask "${SLEEP_TARGETS[@]}" >/dev/null 2>&1 || true

  mkdir -p /etc/systemd/logind.conf.d
  cat > /etc/systemd/logind.conf.d/99-happenstance-bench-host.conf <<'CONF'
# This host is a measurement instrument that happens to have a lid.
[Login]
HandleLidSwitch=ignore
HandleLidSwitchExternalPower=ignore
HandleLidSwitchDocked=ignore
IdleAction=ignore
CONF
  systemctl restart systemd-logind

  note "masking scheduled background work"
  for u in "${NOISY_UNITS[@]}"; do
    systemctl disable --now "$u" >/dev/null 2>&1 || true
    systemctl mask "$u" >/dev/null 2>&1 || true
  done

  note "sysctl"
  cat > /etc/sysctl.d/99-happenstance-bench-host.conf <<'CONF'
# Swapping mid-measurement turns a microsecond arm into a millisecond one. With
# the stacks gone this host has ~14 GiB free; 1 rather than 0 keeps the kernel's
# emergency path available.
vm.swappiness = 1
# `perf` usable without root, so a profile does not require a second decision.
kernel.perf_event_paranoid = 1
CONF
  sysctl --quiet --system

  note "installing the CPU tuning unit"
  install -d "$LIBDIR"
  install -m 0755 "$here/cpu-tuning.sh" "$LIBDIR/cpu-tuning.sh"
  install -m 0644 "$here/host.env"      "$LIBDIR/host.env"
  install -m 0644 "$here/happenstance-bench-tuning.service" \
    /etc/systemd/system/happenstance-bench-tuning.service
  systemctl daemon-reload
  systemctl enable --now happenstance-bench-tuning.service

  # A marker, so `preflight.sh` can tell "this is the reference host and a
  # condition is unmet" from "this is somebody's laptop and none of this
  # applies". Without it, adding preflight to `benchmarks/run.sh` — which is
  # `set -euo pipefail` — would break the harness for every contributor.
  printf '%s\n' "$(date -Is) provisioned by ops/host/00-system.sh" \
    > /etc/happenstance-bench-host

  note 'done. run: systemctl status happenstance-bench-tuning'
}

restore() {
  note "restoring"
  systemctl disable --now happenstance-bench-tuning.service >/dev/null 2>&1 || true
  rm -f /etc/systemd/system/happenstance-bench-tuning.service
  systemctl daemon-reload
  systemctl unmask "${SLEEP_TARGETS[@]}" >/dev/null 2>&1 || true
  for u in "${NOISY_UNITS[@]}"; do
    systemctl unmask "$u" >/dev/null 2>&1 || true
  done
  rm -f /etc/systemd/logind.conf.d/99-happenstance-bench-host.conf
  rm -f /etc/sysctl.d/99-happenstance-bench-host.conf
  rm -f /etc/happenstance-bench-host
  systemctl restart systemd-logind
  note "packages left installed on purpose — an installed package is not a measured condition"
}

case "${1:---apply}" in
  --apply)   apply ;;
  --restore) restore ;;
  *) die "usage: sudo $0 [--apply|--restore]" ;;
esac
