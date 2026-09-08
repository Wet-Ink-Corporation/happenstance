-- Migration 2 — the projection store's checkpoint.
--
-- The number is this directory's sequence, so that a DBA applying the files in
-- lexical order gets a working database for both roles. It is NOT this store's
-- own version: `projection_store::SCHEMA_VERSION` is 1, because this is the
-- first migration of the projection role, which has its own lineage and its own
-- constant exactly as `happenstance-sqlite` does. The two roles are separated by
-- features and a consumer taking only `event-store` never applies this file.
--
-- Applied by `migration::apply_projection`, gated behind `projection-store`, and
-- idempotent for the same reason migration 1 is: the fixture creates one schema
-- per instance and connects onto it more than once, and a migration that fails
-- on the second connect reports a broken environment as an adapter defect.
--
-- The suite's own probe read model is deliberately NOT here. It lives as a
-- constant behind the `conformance` feature, because a test table shipped inside
-- an application's database is a defect no test in this repository could catch --
-- every test enables the feature. A `.sql` file in this directory is, by
-- construction, something a DBA applies.

CREATE TABLE IF NOT EXISTS projection_checkpoint (
    -- The `ProjectionId`, verbatim. `text` rather than a bounded `varchar`: the
    -- contract validates the identifier and this column is not the place to
    -- restate a limit that would then disagree with it.
    projection_id text   NOT NULL PRIMARY KEY,

    -- A `SequencePosition`, which is a `NonZeroU64`, in a signed `bigint`. The
    -- gap is real and it is read back through a fallible conversion --
    -- `PostgresProjectionStoreError::CheckpointOutOfRange` -- rather than by a
    -- cast that would turn a corrupt row into a plausible position. The column
    -- carries the cheap half of the same guard so that a wrong value cannot be
    -- written in the first place.
    position      bigint NOT NULL CHECK (position > 0),

    -- STORED, never inferred. `Checkpoint` is three states and `Authority` is
    -- two, and the third state -- `NeverRun` -- is the ABSENCE of this row.
    -- That is why `reset` deletes rather than writing a sentinel: a sentinel
    -- position would make a reset indistinguishable from a commit at the first
    -- position, which is the defect PS-16's own rule exists to reject.
    --
    -- Text rather than an enum type or an integer. A `CREATE TYPE` is a second
    -- schema object to migrate and to `IF NOT EXISTS` around (which Postgres
    -- does not offer for types), and the value is read by a human with a
    -- debugger far more often than by this module. The CHECK is what keeps it
    -- honest, and it is the constraint an integer column could not carry
    -- legibly.
    authority     text   NOT NULL CHECK (authority IN ('live', 'rebuilding'))
);
