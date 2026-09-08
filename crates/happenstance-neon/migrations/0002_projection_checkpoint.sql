-- Migration 2 — the projection store's checkpoint.
--
-- The number is this directory's sequence, so that an operator applying the
-- files in lexical order gets a working database for both roles. It is NOT this
-- store's own version: this is the first migration of the projection role, which
-- has its own lineage exactly as `happenstance-sqlite` and `happenstance-postgres`
-- do. The two roles are separated by features and a consumer taking only
-- `event-store` never applies this file.
--
-- Templated and split for the reasons migration 1 states in full: there is no
-- session to hold a `search_path` in, and the endpoint's `query` field takes
-- exactly one statement.
--
-- The suite's own probe read model is deliberately NOT here. It lives as a
-- constant behind the `conformance` feature, because a test table shipped inside
-- an application's database is a defect no test in this repository could catch —
-- every test enables the feature. A `.sql` file in this directory is, by
-- construction, something an operator applies.

CREATE SCHEMA IF NOT EXISTS @schema@;

CREATE TABLE IF NOT EXISTS @checkpoint@ (
    -- The `ProjectionId`, verbatim. `text` rather than a bounded `varchar`: the
    -- contract validates the identifier and this column is not the place to
    -- restate a limit that would then disagree with it.
    projection_id text   NOT NULL PRIMARY KEY,

    -- A `SequencePosition`, which is a `NonZeroU64`, in a signed `bigint`. The
    -- gap is real and is read back through a fallible conversion rather than by
    -- a cast that would turn a corrupt row into a plausible position. The column
    -- carries the cheap half of the same guard so that a wrong value cannot be
    -- written in the first place.
    position      bigint NOT NULL CHECK (position > 0),

    -- STORED, never inferred. `Checkpoint` is three states and `Authority` is
    -- two, and the third state — `NeverRun` — is the ABSENCE of this row. That
    -- is why `reset` deletes rather than writing a sentinel: a sentinel position
    -- would make a reset indistinguishable from a commit at the first position.
    --
    -- Text rather than an enum type. A `CREATE TYPE` is a second schema object
    -- to migrate and one Postgres offers no `IF NOT EXISTS` for, and the value
    -- is read by a human with a debugger far more often than by this module.
    authority     text   NOT NULL CHECK (authority IN ('live', 'rebuilding'))
);
