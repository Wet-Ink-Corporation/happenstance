# Runs one test-binary launch under one processor affinity mask.
#
# `taskset` does not exist on Windows, and the three obvious replacements each
# have a defect this experiment cannot afford:
#
#   * `Start-Process -PassThru` then `$p.ProcessorAffinity = …` leaves a window in
#     which the child is already running unconstrained. A seeding phase that ran
#     on twenty cores before the mask landed would produce a "two-core" row that
#     is nothing of the kind.
#   * `cmd /c start /affinity <hex> /b …` sets the mask at creation and is
#     correct, but `start /b` needs a console: invoked from a Git Bash pipeline
#     it produces no output and no process at all, silently.
#   * `SetProcessAffinityMask` through FFI needs `unsafe`, which this crate
#     forbids (`#![forbid(unsafe_code)]`, matching the workspace's
#     `unsafe_code = "forbid"`).
#
# So: **inheritance**. This script sets its own affinity first, then starts the
# test binary, which inherits the mask at creation the way every Windows child
# does. There is no window and no console requirement.
#
# Inheritance is not taken on trust. `HS_EXPECTED_AFFINITY` carries the mask into
# the child, `busy_timeout_margin_probes::cores` reads the child's *own* mask back
# out of the operating system, and every test refuses to emit a figure if the two
# disagree. `results/raw/cores-*.txt` also carries an independent measured
# speedup, which would catch a mask that was reported and not enforced.

[CmdletBinding()]
param(
    # The affinity mask, as a decimal integer. Bit *n* is logical processor *n*.
    [Parameter(Mandatory)][int64]$Mask,
    # The test binary to launch.
    [Parameter(Mandatory)][string]$Exe,
    # Where its stdout is written. Its stderr goes beside it, with `.err`.
    [Parameter(Mandatory)][string]$Out,
    # libtest's own arguments, as one space-separated string.
    #
    # One string rather than an array, because every argument that has to reach
    # libtest begins with `-` — `--nocapture`, `--skip`, `--exact` — and
    # PowerShell binds a leading-dash token to a *parameter name* before it will
    # consider it a value. Splitting here is the spelling that survives both
    # PowerShell's parameter binder and Git Bash's argument rewriting.
    [string]$TestArgs = ''
)

$ErrorActionPreference = 'Stop'

$env:HS_EXPECTED_AFFINITY = "$Mask"

# The mask goes on this process, before the child exists. Every process Windows
# creates inherits its parent's affinity.
(Get-Process -Id $PID).ProcessorAffinity = [IntPtr]$Mask

$argList = @($TestArgs -split '\s+' | Where-Object { $_ -ne '' })

$err = "$Out.err"
$process = Start-Process -FilePath $Exe -ArgumentList $argList -NoNewWindow -PassThru `
    -RedirectStandardOutput $Out -RedirectStandardError $err
$process.WaitForExit()

if (Test-Path $Out) { Get-Content $Out }
if ($process.ExitCode -ne 0) {
    if (Test-Path $err) { Get-Content $err }
    exit $process.ExitCode
}

# A clean launch writes nothing to stderr, and an empty file beside every raw row
# is thirty pieces of noise in a directory whose whole job is to be read. The
# file is kept whenever it has content, which is the only case anyone wants it.
if ((Test-Path $err) -and ((Get-Item $err).Length -eq 0)) { Remove-Item $err }
