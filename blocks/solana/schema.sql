-------------------------------------------------
-- Meta tables to store Substreams information --
-------------------------------------------------
CREATE TABLE IF NOT EXISTS cursors
(
    id        String,
    cursor    String,
    block_num Int64,
    block_id  String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);

-------------------------------------------------
-- Solana block header --
-------------------------------------------------
CREATE TABLE IF NOT EXISTS blocks
(
    -- clock --
    time                    DateTime64(3, 'UTC'),
    number                  UInt64,
    date                    Date,
    hash                    String COMMENT 'Hash',

    -- block --
    slot                    UInt64,
    parent_hash             String COMMENT 'Hash',
    parent_slot             UInt64,

    -- counters --
    -- size                    UInt64,
    total_transactions      UInt64,
    successful_transactions UInt64,
    failed_transactions     UInt64,
    total_instructions      UInt64,
    total_rewards           UInt64,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (hash)
        ORDER BY (hash)
        COMMENT 'Solana block header';


-- Projections --
-- https://clickhouse.com/docs/en/sql-reference/statements/alter/projection --
ALTER TABLE blocks ADD PROJECTION IF NOT EXISTS blocks_by_block_number (
    SELECT * ORDER BY date, number
);


ALTER TABLE blocks MATERIALIZE PROJECTION blocks_by_block_number;
