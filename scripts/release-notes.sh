#!/usr/bin/env bash
# Prints the GitHub release notes for one tag: that version's section of
# CHANGELOG.md, without its heading, and a footer.
#
#   scripts/release-notes.sh v0.5.1 [CHANGELOG.md] > notes.md
#
# `release.yml` runs it to write the body of the GitHub release it creates
# after publishing, and the same script backfills a tag that has none. One
# definition, so a backfilled release and a new one read alike.
#
# It fails, printing nothing to stdout, when the changelog has no
# `## [X.Y.Z]` heading for the tag or the section under it is empty: a release
# whose notes say nothing is worse than a release job that goes red.
set -euo pipefail

tag="${1:?usage: release-notes.sh vX.Y.Z [CHANGELOG.md]}"
changelog="${2:-CHANGELOG.md}"
version="${tag#v}"

# From the line after `## [version]` up to, not including, the next `## [`,
# with the blank lines at either end trimmed. `index(...) == 1` matches the
# heading literally, so the dots in a version are not regex wildcards and
# `0.5.1` does not also match `0.5.10`'s heading (the `]` closes the name).
# Plain POSIX awk, no `tac`, so it runs on a macOS checkout as it does in CI.
body="$(awk -v heading="## [${version}]" '
  index($0, heading) == 1 { found = 1; next }
  found && /^## \[/ { exit }
  found { lines[++n] = $0; if ($0 ~ /[^[:space:]]/) { if (!first) first = n; last = n } }
  END { for (i = first; first && i <= last; i++) print lines[i] }
' "$changelog")"

if [ -z "$body" ]; then
  echo "release-notes.sh: no '## [${version}]' section with content in ${changelog}" >&2
  exit 1
fi

printf '%s\n\n---\n\nEvery crate in this release is published to crates.io under the same number. ' "$body"
printf 'The whole history is in [`CHANGELOG.md`](https://github.com/Wet-Ink-Corporation/happenstance/blob/%s/CHANGELOG.md).\n' "$tag"
