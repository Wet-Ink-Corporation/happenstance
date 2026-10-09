#!/usr/bin/env bash
#
# Applies .github/tracker.yml to the GitHub repository: creates or updates every
# label and milestone it names. Running it twice changes nothing the second time.
#
# ## Why this is a script and not a workflow
#
# The vocabulary changes rarely, and a workflow that edits repository settings
# would need a token with `issues: write` sitting in CI for a job that runs a few
# times a year. So the owner runs this by hand, with their own `gh` login.
#
# ## Running it
#
#   scripts/tracker-apply.sh                  # wet-ink-corporation/happenstance
#   REPO=owner/name scripts/tracker-apply.sh
#   DRY_RUN=1 scripts/tracker-apply.sh        # print, change nothing
#
# It needs `gh` (authenticated, with write access to the repository) and `yq`
# (mikefarah/yq v4, which reads `.github/tracker.yml`).
#
# ## What it does not do
#
# Issue types are an organisation setting: Settings → Planning → Issue types, which
# needs an org admin. The four names it should hold are listed at the bottom of
# tracker.yml. This script prints them as a reminder and does not touch them.
#
# Labels this file does not name are left alone. Removing GitHub's default labels
# is a choice the owner makes, not a side effect of this script.

set -euo pipefail

REPO="${REPO:-wet-ink-corporation/happenstance}"
FILE="$(dirname "$0")/../.github/tracker.yml"

run() {
    if [[ -n "${DRY_RUN:-}" ]]; then
        printf 'would run:'
        printf ' %q' "$@"
        printf '\n'
    else
        "$@"
    fi
}

for tool in gh yq; do
    command -v "$tool" >/dev/null || { echo "error: $tool is not installed" >&2; exit 1; }
done

echo "labels → $REPO"
count="$(yq '.labels | length' "$FILE")"
for ((i = 0; i < count; i++)); do
    name="$(yq -r ".labels[$i].name" "$FILE")"
    color="$(yq -r ".labels[$i].color" "$FILE")"
    description="$(yq -r ".labels[$i].description" "$FILE")"
    # --force updates an existing label's colour and description instead of
    # failing on it, which is what makes the script safe to run twice.
    run gh label create "$name" --repo "$REPO" --color "$color" \
        --description "$description" --force
done

echo "milestones → $REPO"
existing="$(gh api "repos/$REPO/milestones?state=all&per_page=100" --jq '.[].title')"
count="$(yq '.milestones | length' "$FILE")"
for ((i = 0; i < count; i++)); do
    title="$(yq -r ".milestones[$i].title" "$FILE")"
    description="$(yq -r ".milestones[$i].description" "$FILE")"
    if grep -qxF "$title" <<<"$existing"; then
        number="$(gh api "repos/$REPO/milestones?state=all&per_page=100" \
            --jq ".[] | select(.title == \"$title\") | .number")"
        run gh api --method PATCH "repos/$REPO/milestones/$number" \
            -f description="$description" --silent
    else
        run gh api --method POST "repos/$REPO/milestones" \
            -f title="$title" -f description="$description" --silent
    fi
done

echo
echo "issue types (org-level, by hand, needs an org admin):"
yq -r '.issue_types[] | "  " + .name + " — " + .description' "$FILE"
