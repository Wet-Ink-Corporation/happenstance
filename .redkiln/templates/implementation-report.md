---
item: "{{item}}"
stage: implement
created: "{{created}}"
updated: "{{updated}}"
---

# Implementation Report — {{title}}

## TDD Evidence

Which tests went red then green, mapped to each AC.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
|    |      |             |

## Commits

The checkpoint commits this story shipped as. Record them on the item too —
`redkiln record-links <item> --sha <shas>` — before the advance that files this
report: the sha of a commit containing the item file cannot be inside that item
file, so `links.commits` records the WORK commits, which exist by now.

Written here as well as on the item because this is where a reader looks, and
because the field corpus proves the data has always existed in a report and
never anywhere queryable: consumer repos recorded eight-character SHAs in an
`_implementation.md` redkiln had never heard of, while `links.commits` was empty
on every one of 150 items.

| SHA | Subject |
| --- | ------- |
|     |         |

## Changes

The files touched and the shape of the change.

## Gates

Result of test / typecheck / lint / build.

## Notes

Deviations from the plan and why.
