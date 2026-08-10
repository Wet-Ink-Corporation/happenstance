---
item: "{{item}}"
stage: storymap
created: "{{created}}"
updated: "{{updated}}"
---

# Story Map — {{title}}

## Backbone

The user activities/outcomes across the top.

## Slices

The thin vertical slices (stories) under each activity, grouped by the milestone/slice they are delivered
with. Stories sharing a Milestone are implemented together in a single context and mounted as ONE integrated
surface (the implement stage runs one milestone at a time). Cross-milestone `depends_on` edges must be acyclic.
Type each story `capability` (a user-observable slice) or `foundation` (real in-tree substrate a capability
slice in this initiative consumes — never owned outside it, never a double/fixme).

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --------- | ----- | --------- | -------------- | ---------- | --------- |
|           |       |           |                |            |           |

## Merge order

The milestones in dependency order (foundation stories before the capability slices that consume them), and
the stories within each.
