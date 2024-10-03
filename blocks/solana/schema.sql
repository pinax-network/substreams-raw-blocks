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

CREATE TABLE IF NOT EXISTS rewards
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- block --
    block_slot                  UInt64,
    block_parent_hash           String,
    block_parent_slot           UInt64,

    -- reward --
    pubkey                      String COMMENT 'Reward destination',
    lamports                    Int64,
    post_balance                UInt64,
    reward_type                 LowCardinality(String),
    commission                  String,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_hash, pubkey, reward_type)
        ORDER BY (block_hash, pubkey, reward_type)
        COMMENT 'Solana rewards';


-- Projections --
-- https://clickhouse.com/docs/en/sql-reference/statements/alter/projection --
ALTER TABLE blocks ADD PROJECTION IF NOT EXISTS blocks_by_block_number (
    SELECT * ORDER BY date, number
);

ALTER TABLE rewards ADD PROJECTION IF NOT EXISTS rewards_by_block_number (
    SELECT * ORDER BY block_date, block_number
);


ALTER TABLE blocks MATERIALIZE PROJECTION blocks_by_block_number;

ALTER TABLE rewards MATERIALIZE PROJECTION rewards_by_block_number;
