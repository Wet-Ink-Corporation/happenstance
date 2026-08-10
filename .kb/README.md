# Knowledge base (`.kb`)

This is your repository's durable knowledge base — the long-lived counterpart to the
`.bklg` backlog. Where the backlog tracks _work in motion_, the KB holds the _settled
understanding_ that work produces and consumes: decisions, concepts, playbooks, and maps.

The closeout stage of an initiative (`/redkiln:closeout`) harvests durable value into here,
and `/redkiln:kb-ingest` promotes staged documents from `.kb/_intake/` into permanent atoms.

## Atoms

Each unit of knowledge is one **atom**: a markdown file with YAML frontmatter validated by
`redkiln validate --kb` (`KbFrontmatter`, `src/schema/kb.ts`). Copy `_templates/atom.md` to a
new file and fill it in. Required fields:

| Field                    | Meaning                                                                                                               |
| ------------------------ | --------------------------------------------------------------------------------------------------------------------- |
| `id`                     | Unique, stable atom id (e.g. `kb-decision-0582`, `concept-id-allocation`).                                            |
| `title`                  | Human-readable title.                                                                                                 |
| `kind`                   | `narrative` · `concept` · `decision` · `reference` · `playbook` · `map` · `roadmap` · `open_question` · `governance`. |
| `status`                 | `draft` · `proposed` · `accepted` · `superseded` · `withdrawn`.                                                       |
| `authority_tier`         | How binding this is. One documented vocabulary — see below.                                                           |
| `summary`                | One-paragraph summary.                                                                                                |
| `depends_on` / `related` | Ids of other atoms this builds on / relates to.                                                                       |
| `source_paths`           | Backlog items / files this knowledge was distilled from.                                                              |
| `last_reviewed`          | ISO date this atom was last reviewed.                                                                                 |

Decision atoms may also carry `adr_id`, `reversibility` (`low`/`medium`/`high`), `phase`,
`supersedes`, and `superseded_by`.

Unlike `kind` and `status`, `authority_tier` is not an enum — the schema is `z.string().min(1)`,
so `validate --kb` accepts any non-empty string and nothing enforces the vocabulary. That makes it
a convention, and a convention needs exactly one home: the `authority_tier` section of the
[KB atom schema reference](https://wet-ink-corporation.github.io/redkiln/reference/kb-schema/).
Use a tier from there rather than inventing one locally.

## Rules

- **Accepted decisions are immutable.** Once a `decision` atom reaches `status: accepted`, you
  do not edit its substance — you write a new atom that `supersedes` it and set the old one's
  `status: superseded` + `superseded_by`. `redkiln validate --kb` flags edits to an accepted
  decision.
- **Prefer merge-and-link over new files.** When new knowledge fits an existing atom, amend it
  and link, rather than spawning near-duplicates.
- **No dangling links.** Every `depends_on` / `related` / `supersedes` / `superseded_by` id
  must resolve to a real atom — `validate --kb` enforces this.

## Suggested layout

Organize by kind as the corpus grows — e.g. `decisions/`, `concepts/`, `playbooks/`,
`maps/`, `reference/`. `_`-prefixed directories are reserved (`_intake/` staging,
`_templates/`) and are skipped by `validate --kb`.

The directories below are scaffolded for you, each with its own README saying what belongs in it
and what does not. Read that README before filing an atom there — the pipeline routes into these
layers by name, so a repo that invents its own meaning for one of them diverges silently:

- **`product/`** — the durable **personas** (`concept` atoms) and **journeys** (`playbook`
  atoms) an initiative is _for_, both carrying `authority_tier: product`. The initiative
  charter cites them; closeout promotes them here.
- **`design/`** — the resolved **interaction-pattern decisions** a human signed off during a
  project's `design` stage (`concept` atoms, `authority_tier: design`): the pattern chosen for
  a class of surface, the alternatives rejected, the documented failure mode being mitigated,
  and the anti-patterns that proved real.
- **`open-questions/`** — what is deliberately **not** settled (`open_question` atoms,
  `authority_tier: note`), including every conflict `/redkiln:kb-ingest` deferred rather than
  forcing a resolution to. Binds nothing; exists so an absent decision is legible instead of
  being rediscovered.

The first two hold knowledge authored **once** and referenced by many initiatives, and both
previously died inside an archived backlog folder and were re-derived from scratch every round.
For those, harvest the decision and its reasoning, not the pixel layout of one screen — the
pattern chosen for a class of surface, the alternative rejected, and the condition that would
make you choose differently. A copy of last quarter's screen goes stale the moment the code
changes; the judgement behind it does not. None of these directories introduces a new `kind`:
personas and design patterns are `concept` atoms, journeys are `playbook` atoms, and open
questions use the `open_question` kind the schema already declares. What makes `product/` and
`design/` layers rather than kinds is their `authority_tier`.
