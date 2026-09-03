-- `substreams sink postgres` (Relational Mappings Mode) maps every protobuf `string`
-- to VARCHAR(255). Raw block data carries strings well past that (JSON payloads,
-- console output, revert reasons, hex-encoded data...), so after `setup` we widen
-- every such column to TEXT. Postgres stores VARCHAR and TEXT identically; the sink
-- keys its schema check on the protobuf, not on the column types, so this is safe to
-- run before or after loading data, and it is idempotent.
--
--   psql "$DSN" -f sql/varchar_to_text.sql
DO $$
DECLARE
    col record;
BEGIN
    FOR col IN
        SELECT table_schema, table_name, column_name, udt_name
        FROM information_schema.columns
        WHERE table_schema = current_schema()
          AND table_name NOT LIKE '\_%'
          AND (
                (data_type = 'character varying' AND character_maximum_length IS NOT NULL)
             OR (data_type = 'ARRAY' AND udt_name = '_varchar')
          )
    LOOP
        EXECUTE format(
            'ALTER TABLE %I.%I ALTER COLUMN %I TYPE %s',
            col.table_schema, col.table_name, col.column_name,
            CASE WHEN col.udt_name = '_varchar' THEN 'TEXT[]' ELSE 'TEXT' END
        );
    END LOOP;
END $$;
