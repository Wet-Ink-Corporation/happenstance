<!--
Thanks for sending this.

CONTRIBUTING.md is the long version; this is the short one. Delete any section
that does not apply rather than writing "n/a" in it — an empty heading is noise
and a deleted one is a decision.
-->

## What this changes, and why

<!--
The why is the part that is hard to reconstruct later. If this fixes something,
say what the wrong behaviour was, not only what the new behaviour is.
-->

## What it would have taken to catch this

<!--
Only for a fix. Which check should have failed and did not? "None — nothing
watches this" is a complete and useful answer, and often the more valuable half
of the pull request.
-->

## Alternatives that lost

<!--
Optional, and worth more than it looks for anything touching a port, a
conformance rule or the query planner. A rejected option with its reason is what
stops the same option being re-proposed in six months.
-->

## Checklist

- [ ] `cargo xtask ci` is green locally, and I read the exit code rather than a
      notification. (`--fast` is the bar for work in progress; the full gate is
      the bar for merge.)
- [ ] New behaviour has a test that fails without the change. I checked that by
      reverting the implementation and watching it fail, not by assuming.
- [ ] If I added a conformance rule: I can name a plausible wrong implementation
      it rejects, and that implementation exists in the testkit's own `tests/`.
- [ ] If I changed a public surface: the change is intentional and
      `CHANGELOG.md` says what broke and why. `cargo-semver-checks` runs on this
      pull request and is not a formality.
- [ ] If I moved code that anything cites by line: `cargo xtask lint-constitution`
      and `cargo xtask spec-trace` are green. Repoint by **anchor**, never by
      offset — the modal offset leaves the outlier wrong *and* green.

## Anything a reviewer should look at hardest

<!--
Where you are least confident. Saying so is not a weakness in a pull request;
it is the single most useful sentence a reviewer can be given.
-->
