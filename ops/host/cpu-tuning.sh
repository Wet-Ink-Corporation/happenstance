#!/usr/bin/env bash
#
# Applies the CPU regime declared in `host.env`, and restores what was there
# before it.
#
# Run as root, normally by `happenstance-bench-tuning.service` rather than by
# hand — the unit is what makes the tuning survive a reboot and makes
# `systemctl is-active` a question `preflight.sh` can ask. Applying it by hand
# leaves a machine that is tuned and cannot say so.
#
# ---------------------------------------------------------------------------
# Why --restore reads a snapshot rather than writing defaults
# ---------------------------------------------------------------------------
#
# The obvious `--restore` writes `powersave` and `cpuinfo_max_freq` back. It is
# wrong, and wrong in a way nothing would catch: a restore that writes assumed
# defaults leaves a machine that PASSES the next check against those defaults
# while not being what it was. The first `--apply` therefore snapshots every
# value it is about to change into $SNAPSHOT and never overwrites an existing
# one, so `--restore` puts back what was actually found — including the case
# where the machine was already partly tuned when we arrived.

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=./host.env
. "${HOST_ENV:-$here/host.env}"

SNAPSHOT="${SNAPSHOT:-/var/lib/happenstance-bench-host/pre-tuning.env}"

die() { printf 'cpu-tuning: %s\n' "$*" >&2; exit 1; }
note() { printf 'cpu-tuning: %s\n' "$*"; }

[[ $EUID -eq 0 ]] || die "must run as root (it writes /sys)"

online_cpus() {
  local c
  for c in /sys/devices/system/cpu/cpu[0-9]*; do
    [[ -r "$c/cpufreq/scaling_governor" ]] && basename "$c"
  done
}

read_sysfs() { [[ -r "$1" ]] && cat "$1" 2>/dev/null || true; }

take_snapshot() {
  [[ -e "$SNAPSHOT" ]] && { note "snapshot exists, keeping it: $SNAPSHOT"; return; }
  mkdir -p "$(dirname "$SNAPSHOT")"
  {
    echo "# Captured by cpu-tuning.sh --apply at $(date -Is), before the first change."
    echo "# This is what the machine WAS. --restore writes these back verbatim."
    echo "SNAP_SMT=$(read_sysfs /sys/devices/system/cpu/smt/control)"
    echo "SNAP_CLOCKSOURCE=$(read_sysfs /sys/devices/system/clocksource/clocksource0/current_clocksource)"
    echo "SNAP_THP=$(sed -n 's/.*\[\(.*\)\].*/\1/p' /sys/kernel/mm/transparent_hugepage/enabled 2>/dev/null)"
    local cpu n
    for cpu in $(online_cpus); do
      n="${cpu#cpu}"
      echo "SNAP_GOV_$n=$(read_sysfs /sys/devices/system/cpu/$cpu/cpufreq/scaling_governor)"
      echo "SNAP_EPP_$n=$(read_sysfs /sys/devices/system/cpu/$cpu/cpufreq/energy_performance_preference)"
      echo "SNAP_MAX_$n=$(read_sysfs /sys/devices/system/cpu/$cpu/cpufreq/scaling_max_freq)"
    done
  } > "$SNAPSHOT"
  note "snapshot written: $SNAPSHOT"
}

apply() {
  take_snapshot

  # SMT first: it changes which CPUs exist, so every per-CPU write below must
  # see the final set. Doing it last would leave the offlined siblings carrying
  # whatever they had, and onlining them later would resurrect it.
  if [[ -w /sys/devices/system/cpu/smt/control ]]; then
    echo "$SMT" > /sys/devices/system/cpu/smt/control
    note "smt=$SMT"
  fi

  local cpu wrote=0
  for cpu in $(online_cpus); do
    echo "$GOVERNOR" > "/sys/devices/system/cpu/$cpu/cpufreq/scaling_governor"
    [[ -w "/sys/devices/system/cpu/$cpu/cpufreq/energy_performance_preference" ]] &&
      echo "$EPP" > "/sys/devices/system/cpu/$cpu/cpufreq/energy_performance_preference"
    if [[ -n "${MAX_FREQ_KHZ:-}" ]]; then
      echo "$MAX_FREQ_KHZ" > "/sys/devices/system/cpu/$cpu/cpufreq/scaling_max_freq"
    fi
    wrote=$((wrote + 1))
  done
  note "governor=$GOVERNOR epp=$EPP max_freq_khz=${MAX_FREQ_KHZ:-<uncapped>} on $wrote cpus"

  [[ -w /sys/kernel/mm/transparent_hugepage/enabled ]] &&
    echo "$THP" > /sys/kernel/mm/transparent_hugepage/enabled
  note "thp=$THP"

  # The clocksource, and on this host that is a load-bearing line rather than a
  # tidy one. See host.env: with tsc selectable, CLOCK_MONOTONIC goes backwards
  # by up to 2 ms across CPUs on this part, measured. Writing it here means the
  # choice survives a kernel command line someone re-adds, and means preflight
  # can assert a value that something is actually responsible for.
  if [[ -n "${CLOCKSOURCE:-}" && -w /sys/devices/system/clocksource/clocksource0/current_clocksource ]]; then
    if grep -qw "$CLOCKSOURCE" /sys/devices/system/clocksource/clocksource0/available_clocksource; then
      echo "$CLOCKSOURCE" > /sys/devices/system/clocksource/clocksource0/current_clocksource
      note "clocksource=$CLOCKSOURCE"
    else
      note "clocksource=$CLOCKSOURCE NOT AVAILABLE; left at $(read_sysfs /sys/devices/system/clocksource/clocksource0/current_clocksource)"
    fi
  fi
}

restore() {
  [[ -r "$SNAPSHOT" ]] || die "no snapshot at $SNAPSHOT — refusing to guess what this machine was"
  # shellcheck source=/dev/null
  . "$SNAPSHOT"

  # SMT back on first, for the same reason apply turns it off first: the CPUs
  # whose values we are about to restore have to exist.
  if [[ -n "${SNAP_SMT:-}" && -w /sys/devices/system/cpu/smt/control ]]; then
    echo "$SNAP_SMT" > /sys/devices/system/cpu/smt/control
  fi

  local cpu n gov epp max
  for cpu in $(online_cpus); do
    n="${cpu#cpu}"
    gov="SNAP_GOV_$n"; epp="SNAP_EPP_$n"; max="SNAP_MAX_$n"
    [[ -n "${!gov:-}" ]] && echo "${!gov}" > "/sys/devices/system/cpu/$cpu/cpufreq/scaling_governor"
    [[ -n "${!epp:-}" && -w "/sys/devices/system/cpu/$cpu/cpufreq/energy_performance_preference" ]] &&
      echo "${!epp}" > "/sys/devices/system/cpu/$cpu/cpufreq/energy_performance_preference"
    [[ -n "${!max:-}" ]] && echo "${!max}" > "/sys/devices/system/cpu/$cpu/cpufreq/scaling_max_freq"
  done

  [[ -n "${SNAP_THP:-}" && -w /sys/kernel/mm/transparent_hugepage/enabled ]] &&
    echo "$SNAP_THP" > /sys/kernel/mm/transparent_hugepage/enabled

  if [[ -n "${SNAP_CLOCKSOURCE:-}" ]] &&
     grep -qw "$SNAP_CLOCKSOURCE" /sys/devices/system/clocksource/clocksource0/available_clocksource; then
    echo "$SNAP_CLOCKSOURCE" > /sys/devices/system/clocksource/clocksource0/current_clocksource
  fi

  note "restored from $SNAPSHOT"
}

case "${1:---apply}" in
  --apply)   apply ;;
  --restore) restore ;;
  *) die "usage: $0 [--apply|--restore]" ;;
esac
