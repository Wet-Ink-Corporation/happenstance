---
item: "{{item}}"
stage: decomposition
created: "{{created}}"
updated: "{{updated}}"
---

# Decomposition — {{title}}

## Children

The projects (or stories) this item decomposes into.

| Child | Slug | Rationale |
| ----- | ---- | --------- |
|       |      |           |

## Sequencing

The order in which the children should be delivered and why.

## Story map

Only a **lite** project fills this in. `project-lite` has no `storymap` stage and
therefore no `_storymap.md`, so the story map for a lite project lives here —
which is why `plan.md` and `retier --preview` both send you to this section.
Leave the heading empty on a standard project; its story map is `_storymap.md`.

| Slice | Story | Slug | Archetype | Depends on | Traces to |
| ----- | ----- | ---- | --------- | ---------- | --------- |
|       |       |      |           |            |           |

The proposed merge order, slice by slice — foundation stories before the
capability slices that consume them.
