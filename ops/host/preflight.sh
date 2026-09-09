#!/usr/bin/env bash
#
# This asserts on the MACHINE, never on a MEASUREMENT.
#
# Every check below reads a value out of sysfs, systemd, or a --version, and
# compares it against `ops/host/host.env` -- the declared conditions of the
# reference host. Not one of them reads a figure this repository produced. There
# is no budget here, no threshold on any elapsed time, and nothing that gets
# *raised* when a run comes in slow.
#
# CF-34 (`spec/SPECIFICATION.md:8747`) rejects "a benchmark result gating a
# merge". This gates nothing: no `cargo xtask ci` step, no `.redkiln/config.yaml`
# `verify:` command and no CI job invokes it, and `xtask/src/affected.rs`'s INERT
# list names `ops/` so a diff here does not widen the affected gate. Its only
# caller is `ops/host/bench.sh`, which wraps `benchmarks/run.sh` -- itself not a
# gate step (`benchmarks/run.sh:13-20`) -- and its only failure mode is that no
# figure was taken, never that a figure was too large.
#
# The line to hold, in this repository's own words: `benchmarks/src/paired.rs`
# reports drift and refuses to threshold it (`paired.rs:71-73`), because drift is
# computed FROM SAMPLES. Everything here is read BEFORE the first sample exists.
# If you find yourself adding a check whose input is a number this repository
# measured, you are writing the thing CF-34 forbids -- put it in the report.
#
# ---------------------------------------------------------------------------
# Not the reference host is not a failure
# ---------------------------------------------------------------------------
#
# `benchmarks/run.sh` runs on contributor laptops, and it is `set -euo pipefail`.
# If this script exited non-zero on a Windows or macOS checkout, adding it to the
# harness would break the harness for everyone who is not us. So: absent the
# `/etc/happenstance-bench-host` marker, it prints a banner and exits 0.

set -uo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=./host.env
. "${HOST_ENV:-$here/host.env}"

# The toolchain rows below must be asked INSIDE the checkout. `rust-toolchain.toml`
# pins the channel per-directory, and `rustup default` is deliberately `none`
# (10-toolchain.sh installs no default so the pin is the only copy), so outside
# the repository `rustc --version` answers with whatever toolchain rustup falls
# back to -- here, nightly. That is a true answer to the wrong question: what the
# gate will use is what the checkout resolves.
repo="${HAPPENSTANCE_REPO:-$(cd "$here/../.." 2>/dev/null && pwd)}"
in_repo() { if [ -f "$repo/rust-toolchain.toml" ]; then (cd "$repo" && "$@"); else "$@"; fi; }

# rustup installs into ~/.cargo/bin and 10-toolchain.sh deliberately passes
# `--no-modify-path`. A non-interactive `ssh host cmd` does not read the profile
# that would otherwise add it -- Ubuntu's .bashrc returns early when not
# interactive -- so without this every toolchain row below reads ABSENT on a host
# where the toolchain is installed and fine. A preflight that reports a hole it
# invented is worse than one that reports nothing.
case ":$PATH:" in
  *":$HOME/.cargo/bin:"*) ;;
  *) PATH="$HOME/.cargo/bin:$PATH"; export PATH ;;
esac

MARKER=/etc/happenstance-bench-host
MODE="${1:---assert}"
REPORT="${PREFLIGHT_REPORT:-}"

fails=0
rows=()

row() { # kind, name, expected, actual
  rows+=("$(printf '%-6s %-32s %-30s %s' "$1" "$2" "$3" "$4")")
  if [ "$1" = "FAIL" ]; then fails=$((fails + 1)); fi
  return 0
}

check() { # name, expected, actual
  if [ "$2" = "$3" ]; then row "ok" "$1" "$2" "$3"; else row "FAIL" "$1" "$2" "$3"; fi
}

note() { row "note" "$1" "-" "$2"; }

# `${bad:+wrong on:$bad}${bad:-all ok}` looks like an if/else and is not:
# `${x:-default}` yields $x whenever x is non-empty, so a non-empty $bad printed
# the offending list TWICE. Spell the branch out.
either() { # bad-list, prefix, ok-text
  if [ -n "$1" ]; then printf '%s%s' "$2" "$1"; else printf '%s' "$3"; fi
}

read_sysfs() { if [ -r "$1" ]; then tr -d '\n' < "$1"; else echo "<unreadable>"; fi; }

# --- not the reference host -------------------------------------------------
if [ "$(uname -s)" != "Linux" ] || [ ! -e "$MARKER" ]; then
  echo "preflight: this is not the happenstance measurement host."
  echo
  echo "  Nothing below was checked, and that is not a failure. The declared"
  echo "  conditions in ops/host/host.env describe ${HOST_NAME}; figures taken"
  echo "  here are taken under conditions you must state yourself, exactly as"
  echo "  benchmarks/README.md#conditions asks. ops/host/README.md says how to"
  echo "  provision a host of your own."
  echo
  exit 0
fi

# --- 1. the driver the regime is expressed in -------------------------------
check "cpufreq driver" "$EXPECTED_SCALING_DRIVER" \
  "$(read_sysfs /sys/devices/system/cpu/cpu0/cpufreq/scaling_driver)"

# --- 2-4. governor, EPP, cap: EVERY cpu, not a spot check -------------------
# A mixed set is the case a spot check misses, and it is the likely one: a core
# taken offline and brought back comes back at the kernel default.
bad_gov=""
bad_epp=""
bad_max=""
n_cpu=0
for c in /sys/devices/system/cpu/cpu[0-9]*; do
  [ -r "$c/cpufreq/scaling_governor" ] || continue
  n_cpu=$((n_cpu + 1))
  if [ "$(read_sysfs "$c/cpufreq/scaling_governor")" != "$GOVERNOR" ]; then
    bad_gov="$bad_gov ${c##*/}"
  fi
  if [ -r "$c/cpufreq/energy_performance_preference" ]; then
    if [ "$(read_sysfs "$c/cpufreq/energy_performance_preference")" != "$EPP" ]; then
      bad_epp="$bad_epp ${c##*/}"
    fi
  fi
  if [ -n "${MAX_FREQ_KHZ:-}" ]; then
    if [ "$(read_sysfs "$c/cpufreq/scaling_max_freq")" != "$MAX_FREQ_KHZ" ]; then
      bad_max="$bad_max ${c##*/}"
    fi
  fi
done
check "governor (every online cpu)" "$GOVERNOR on all" \
  "$(either "$bad_gov" "wrong on:" "$GOVERNOR on all")"
check "EPP (every online cpu)" "$EPP on all" \
  "$(either "$bad_epp" "wrong on:" "$EPP on all")"
if [ -n "${MAX_FREQ_KHZ:-}" ]; then
  check "scaling_max_freq" "$MAX_FREQ_KHZ" \
    "$(either "$bad_max" "wrong on:" "$MAX_FREQ_KHZ on all")"
else
  note "scaling_max_freq" "uncapped by declaration (host.env MAX_FREQ_KHZ empty)"
fi

# --- 5. SMT and the machine's size ------------------------------------------
# Half the machine silently coming back makes the run valid and about a
# different host.
check "smt/control" "$SMT" "$(read_sysfs /sys/devices/system/cpu/smt/control)"
if [ "$SMT" = "on" ]; then
  check "online cpus" "$EXPECTED_CPUS" "$n_cpu"
else
  check "online cpus" "$((EXPECTED_CPUS / 2))" "$n_cpu"
fi

# --- 6. the regime is APPLIED, and can say so -------------------------------
# This is the condition that makes 2-5 a unit rather than a manual ritual: it is
# the difference between reading a governor somebody set by hand this morning
# and reading one a reboot will set again.
check "tuning unit active" "active" \
  "$(systemctl is-active happenstance-bench-tuning.service 2>/dev/null || echo inactive)"

# --- 7. nothing wakes up mid-run --------------------------------------------
noisy=""
for u in apt-daily.timer apt-daily-upgrade.timer unattended-upgrades.service \
         man-db.timer motd-news.timer fstrim.timer fwupd-refresh.timer; do
  # NOT `... || echo absent`. `systemctl is-enabled` exits NON-ZERO for a
  # masked unit while still printing `masked`, so the `||` arm fires as well and
  # the answer becomes two words -- which read as a FAIL for units that were
  # correctly masked. A preflight that fails on the state it is asking for is
  # worse than no preflight: it teaches the reader to skim the FAIL rows.
  s="$(systemctl is-enabled "$u" 2>/dev/null | head -1)"
  [ -n "$s" ] || s=absent
  case "$s" in
    masked|absent|disabled|static|indirect) ;;
    *) noisy="$noisy $u=$s" ;;
  esac
done
check "scheduled maintenance" "all masked/absent" \
  "$(either "$noisy" "live:" "all masked/absent")"
check "user crontab entries" "0" "$(crontab -l 2>/dev/null | grep -cvE '^[[:space:]]*(#|$)')"

# --- 8. the machine stays awake ---------------------------------------------
slept=""
for t in sleep.target suspend.target hibernate.target hybrid-sleep.target; do
  # Same non-zero-exit-on-masked trap as the loop above.
  if [ "$(systemctl is-enabled "$t" 2>/dev/null | head -1)" != "masked" ]; then
    slept="$slept $t"
  fi
done
check "sleep targets" "all masked" "$(either "$slept" "unmasked:" "all masked")"

# --- 9. on AC ---------------------------------------------------------------
# On battery the platform profile drops the ceiling mid-run without telling
# anyone, and the samples either side of the drop go into the same median.
ac=0
for p in /sys/class/power_supply/A*/online; do
  [ -r "$p" ] && ac="$(cat "$p")"
done
check "on AC power" "1" "$ac"

# --- 10. the clocksource, as an IDENTITY and not as a measured cost ---------
# An hpet/acpi_pm fallback moves the timer floor by an order of magnitude and
# re-labels arms TIMER-DOMINATED for a reason that has nothing to do with the
# code under test. Note carefully: this compares a NAME. Comparing a measured
# timer cost here would be the thing CF-34 forbids -- paired.rs owns that number.
check "clocksource" "$CLOCKSOURCE" \
  "$(read_sysfs /sys/devices/system/clocksource/clocksource0/current_clocksource)"
tscflags="$(grep -o -m1 -E 'constant_tsc|nonstop_tsc' /proc/cpuinfo | sort -u | paste -sd, -)"
check "tsc flags" "constant_tsc,nonstop_tsc" "${tscflags:-none}"

# --- 11. THP ----------------------------------------------------------------
# `always` changes allocator behaviour, and the allocation counts are the column
# benchmarks/README.md tells readers to quote in preference to the wall clock.
check "transparent_hugepage" "$THP" \
  "$(sed -n 's/.*\[\(.*\)\].*/\1/p' /sys/kernel/mm/transparent_hugepage/enabled 2>/dev/null)"

# --- 12. where the SQLite arms will actually write ---------------------------
# On a host where the temp dir is a tmpfs they measure RAM and report it in a
# table headed by a filesystem. The Linux analogue of the Windows conditions row
# that pins %TEMP% to the internal NVMe.
tmp="${TMPDIR:-/tmp}"
check "tmpdir fstype ($tmp)" "$TMPDIR_FSTYPE" \
  "$(findmnt -no FSTYPE --target "$tmp" 2>/dev/null || echo unknown)"
free_gib="$(df -BG --output=avail "$tmp" 2>/dev/null | tail -1 | tr -dc '0-9')"
if [ -n "$free_gib" ] && [ "$free_gib" -ge "$MIN_FREE_GIB" ]; then
  row "ok" "tmpdir free space" ">= ${MIN_FREE_GIB}G" "${free_gib}G"
else
  row "FAIL" "tmpdir free space" ">= ${MIN_FREE_GIB}G" "${free_gib:-unknown}G"
fi

# --- 13. quiet ---------------------------------------------------------------
load1="$(cut -d' ' -f1 /proc/loadavg)"
if awk -v l="$load1" -v m="$MAX_LOADAVG_1" 'BEGIN{exit !(l<=m)}'; then
  row "ok" "loadavg 1m" "<= $MAX_LOADAVG_1" "$load1"
else
  row "FAIL" "loadavg 1m" "<= $MAX_LOADAVG_1" "$load1"
fi

running="$(docker ps --format '{{.Names}}' 2>/dev/null | sort | paste -sd, -)"
check "running containers" "$ALLOWED_CONTAINERS" "${running:-<none>}"
note "logged-in sessions" "$(who 2>/dev/null | wc -l | tr -d ' ') (this one included)"

# --- 14. thermal, PRE-RUN ONLY ----------------------------------------------
# Deliberately a precondition and never a post-run verdict. Temperature consulted
# afterwards to decide whether figures count is a threshold on a result wearing a
# thermometer. bench.sh records the throttle counters after a run; it does not
# judge them.
pkg_temp="$(sensors -u 2>/dev/null | awk '/_input/{print int($2); exit}')"
if [ -n "$pkg_temp" ]; then
  if [ "$pkg_temp" -le "$MAX_PKG_TEMP_C" ]; then
    row "ok" "package temp (pre-run)" "<= ${MAX_PKG_TEMP_C}C" "${pkg_temp}C"
  else
    row "FAIL" "package temp (pre-run)" "<= ${MAX_PKG_TEMP_C}C" "${pkg_temp}C"
  fi
else
  note "package temp (pre-run)" "no sensor reading"
fi

# --- 15. the toolchain the gate needs PRESENT rather than skipped ------------
# `cargo xtask ci` skips an optional step when its probe finds no tool, and says
# `skipped:`. On this host a skip is a hole, not a courtesy.
check "rustc (in checkout)" "$RUSTC_VERSION" "$(in_repo rustc --version 2>/dev/null | awk '{print $2}')"
check "cargo-hack" "present" \
  "$(in_repo cargo hack --version >/dev/null 2>&1 && echo present || echo ABSENT)"
check "cargo-deny" "present" \
  "$(in_repo cargo deny --version >/dev/null 2>&1 && echo present || echo ABSENT)"
check "nightly toolchain" "present" \
  "$(RUSTUP_AUTO_INSTALL=0 cargo +nightly --version >/dev/null 2>&1 && echo present || echo ABSENT)"
check "wasm-bindgen-test-runner" "present" \
  "$(command -v wasm-bindgen-test-runner >/dev/null 2>&1 && echo present || echo ABSENT)"
check "docker daemon" "up" \
  "$(docker info >/dev/null 2>&1 && echo up || echo DOWN)"

# --- 16. tree state: REPORTED, never rejected --------------------------------
# results/history/ filenames already carry `-dirty`; the record handles it, and
# refusing to measure a dirty tree would refuse the one measurement most worth
# taking -- the one you are in the middle of changing something for.
if (cd "$repo" 2>/dev/null && git rev-parse --git-dir >/dev/null 2>&1); then
  if [ -n "$(cd "$repo" && git status --porcelain 2>/dev/null)" ]; then tree=dirty; else tree=clean; fi
  note "git tree" "$(cd "$repo" && git rev-parse --short HEAD) $tree"
fi

# --- output ------------------------------------------------------------------
emit() {
  echo "preflight: $HOST_NAME -- $HOST_DESCRIPTION"
  echo "preflight: $(date -Is)  kernel $(uname -r)"
  echo
  printf '%s\n' "${rows[@]}"
  echo
  if [ "$fails" -eq 0 ]; then
    echo "preflight: all declared conditions met."
  else
    echo "preflight: $fails condition(s) unmet -- see FAIL rows above."
  fi
}

# The report is written BEFORE the exit status is decided. A refused run must
# leave its reason on disk, or the next person sees only a non-zero exit.
if [ -n "$REPORT" ]; then
  mkdir -p "$(dirname "$REPORT")"
  emit > "$REPORT"
fi
emit

case "$MODE" in
  --report) exit 0 ;;
  --assert) [ "$fails" -eq 0 ] || exit 1 ;;
  *) echo "usage: $0 [--assert|--report]" >&2; exit 2 ;;
esac
