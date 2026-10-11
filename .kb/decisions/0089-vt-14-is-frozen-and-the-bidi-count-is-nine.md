---
id: kb-decision-0089
title: VT-14 is frozen on the RTL corpus run, and the explicit bidirectional controls are nine, not seven
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0089
reversibility: low
phase: 17b
supersedes: null
superseded_by: null
summary: >-
  The owner's call, 2026-10-11 ("Freeze, fix count"). VT-14 moves from PROVISIONAL to FROZEN on
  the run in experiments/identifier-validation/ (#86, PR #239), whose "empty" criterion was
  committed before any harness code. The shipped validator, read through EventType::new and
  Tag::new, agreed with the oracle on all 1,112,064 scalar values in each of three positions;
  it refused 74 values alone, none in any of 19 right-to-left blocks; and all 51 Arabic, Hebrew
  and Persian corpus entries were accepted, with no negative control escaping. That was the
  freeze-by-17b condition ADR-0066 set and ADR-0072 moved. The same run found a miscount: the
  clause, ADR-0015 and the crate's doc comments said "the seven explicit bidirectional formatting
  controls", but the ranges they name, U+202A–U+202E and U+2066–U+2069, hold nine (five embedding
  and override controls, four isolates), and validate.rs has always refused nine. The behaviour
  does not change. The live prose now says nine, and the validator's unit test is renamed to
  rejects_all_nine_bidirectional_controls. ADR-0015's atom and long-form record keep "seven",
  because an accepted record's body is immutable; this atom is the correction they point to.
  Reopened by a legitimate event type or tag that requires an explicit bidirectional control.
depends_on:
  - kb-decision-0066
  - kb-decision-0072
related:
  - kb-decision-0015
  - kb-decision-0066
  - kb-decision-0072
  - kb-decision-0082
source_paths:
  - experiments/identifier-validation/README.md
  - experiments/identifier-validation/results/raw/run.txt
  - crates/happenstance-core/src/validate.rs
  - spec/SPECIFICATION.md
  - runbook/ledgers.md
last_reviewed: 2026-10-11
---

# VT-14 is frozen on the RTL corpus run, and the explicit bidirectional controls are nine, not seven

**Accepted by the owner on 2026-10-11** ("Freeze, fix count"). No long-form record exists; this
atom is the record.

## Context

VT-14 says `EventType::new` and `Tag::new` reject `Cc` and the explicit bidirectional formatting
controls, and must not reject `Cf` generally. Its marker was falsified by *a legitimate event
type or tag that requires an explicit bidirectional formatting control*. ADR-0066 refused to
freeze it because ADR-0015's experiment E11 had tested how the check is built, not whether any
legitimate value needs a bidi control, and E11 was not in `experiments/`
(`references/adr/0066-what-1-0-promises.md:216-218`). It gave the clause a condition instead: an
RTL identifier corpus check, Arabic, Hebrew and Persian with mixed LTR, comes back empty, with
E11's reproduction added to `experiments/`. ADR-0072 moved that row from `freeze-by-17` to
`freeze-by-17b`, because the answer is additive.

`experiments/identifier-validation/` (issue #86, PR #239) is that check. Its criterion was
committed before any harness code (`337d6a29`), and it ran against the shipped validator through
the public constructors rather than a copy of the walk.

## Decision

1. **VT-14 is `[FROZEN]`.** The run came back empty on every pre-registered criterion
   (`experiments/identifier-validation/README.md`, "Result"):
   - 0 disagreements with the oracle in 1,112,064 scalar values, in each of three positions
     (alone, between ASCII letters, between two Arabic letters), for both constructors;
   - 74 values refused alone, exactly the 65 `Cc` codepoints and the bidirectional ranges, and
     none inside any of the 19 right-to-left blocks the harness counts;
   - 51 corpus entries (Arabic 21, Hebrew 16, Persian 14), each as an event type and a tag, and
     10 `key:value` pairs through `Tag::key_value`: 0 refused and 0 altered;
   - none of the 1,530 negative controls per constructor escaped, so the harness could see a
     refusal.
2. **The count is nine.** U+202A–U+202E is five codepoints (LRE, RLE, PDF, LRO, RLO) and
   U+2066–U+2069 is four (LRI, RLI, FSI, PDI). `validate.rs` matches `E2 80 AA`–`E2 80 AE` and
   `E2 81 A6`–`E2 81 A9`, which is those nine, and its unit test has always walked nine. The
   ranges were always the criterion and they do not change. Only the word "seven" was wrong.
3. **What is corrected, and what is not.** The live prose now says nine: VT-14 and its
   rationale in `spec/SPECIFICATION.md`; the doc comments on `EventType::new`, `Tag::new`,
   `validate::check`, `Refusal::BidirectionalControl` and both errors'
   `BidirectionalControl` variants. The unit test `rejects_all_seven_bidirectional_controls` is
   renamed `rejects_all_nine_bidirectional_controls`, and its body is unchanged. These records
   keep "seven", because they are immutable or are history:
   - ADR-0015's atom, at `.kb/decisions/0015-validated-identifiers-and-store-limits.md:15`,
     `:63` and `:112`;
   - ADR-0015's long-form record, at
     `references/adr/0015-validated-identifiers-and-store-limits.md:160`, `:190`, `:199`, `:220`,
     `:876` and `:957`;
   - the evaluations under `references/evaluation/`, the intake records under
     `.kb/_governance/`, and the raw test logs under `experiments/`, which quote the old test
     name.

   Read "seven" in any of them as the nine codepoints of the two ranges it names.

## Consequences

- From `1.0.0`, adding a codepoint to VT-14's refused set, or removing one, is a major release.
  It reaches `ProjectionId` too, because VT-35 applies VT-14's set by reference.
- An empty corpus run is necessary for this freeze, and the experiment says it is not
  sufficient. The corpus has one author and is a sample, not a population. The falsifier stays
  reachable by any value an application produces later.
- The invisible `Cf` characters VT-14 leaves legal, such as U+200B, stay a hazard, as the clause
  already says. The corpus showed RTL text uses `Cf`: ZWNJ, ZWJ, LRM, RLM and ALM all appear in
  it, and all are accepted. A blanket `Cf` ban would have refused real Persian entries.

## What reopens it

A legitimate event type or tag that requires an explicit bidirectional formatting control. If one
appears after `1.0.0`, narrowing the refused set is a breaking change and needs its own record.

## Alternatives that lost

- **Renew the marker past 1.0.** This would keep the falsifier open with nothing left to
  measure. It lost because the condition ADR-0066 set for freezing was met.
- **Freeze and leave the count.** The clause would then freeze a sentence that contradicts its
  own ranges and the shipped code. The owner chose to fix it in the same change.
