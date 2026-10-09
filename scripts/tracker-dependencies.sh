#!/usr/bin/env bash
#
# Applies native GitHub "blocked by" links from a TSV file of
# `<blocked issue>\t<blocker issue>[\t…]` rows. Lines starting with `#` are
# comments. Running it twice changes nothing the second time: a link that
# already exists is skipped.
#
# ## Why this is a script
#
# The GitHub tools an agent session has can create issues and sub-issues, but
# not dependency links. So the owner runs this once after a seeding, with their
# own `gh` login, the same way as `scripts/tracker-apply.sh`.
#
# ## Running it
#
#   scripts/tracker-dependencies.sh references/evaluation/backlog-2026-10-09/dependencies.tsv
#   REPO=owner/name scripts/tracker-dependencies.sh <file>
#   DRY_RUN=1 scripts/tracker-dependencies.sh <file>     # print, change nothing
#
# GitHub allows at most 50 "blocked by" links per issue. The seeded file needs
# at most 7, so the limit is not checked here.

set -euo pipefail

REPO="${REPO:-wet-ink-corporation/happenstance}"
FILE="${1:?usage: tracker-dependencies.sh <dependencies.tsv>}"

command -v gh >/dev/null || { echo "error: gh is not installed" >&2; exit 1; }

added=0
skipped=0
while IFS=$'\t' read -r blocked blocker _; do
    [[ -z "$blocked" || "$blocked" == \#* ]] && continue

    existing="$(gh api "repos/$REPO/issues/$blocked/dependencies/blocked_by?per_page=100" \
        --jq '.[].number')"
    if grep -qxF "$blocker" <<<"$existing"; then
        skipped=$((skipped + 1))
        continue
    fi

    # The endpoint takes the blocker's numeric id, not its issue number.
    blocker_id="$(gh api "repos/$REPO/issues/$blocker" --jq '.id')"
    if [[ -n "${DRY_RUN:-}" ]]; then
        echo "would link: #$blocked blocked by #$blocker (id $blocker_id)"
    else
        gh api --method POST "repos/$REPO/issues/$blocked/dependencies/blocked_by" \
            -F issue_id="$blocker_id" --silent
        echo "linked: #$blocked blocked by #$blocker"
    fi
    added=$((added + 1))
done <"$FILE"

echo "done: $added linked, $skipped already present"
