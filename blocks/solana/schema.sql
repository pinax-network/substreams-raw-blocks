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
    time                            DateTime64(3, 'UTC'),
    date                            Date,
    hash                            String COMMENT 'Hash',

    -- block --
    slot                            UInt64,
    height                          UInt64,
    previous_block_hash             String COMMENT 'Hash',
    parent_slot                     UInt64,

    -- counters --
    total_transactions              UInt64,
    successful_transactions         UInt64,
    failed_transactions             UInt64,
    total_vote_transactions         UInt64,
    total_non_vote_transactions     UInt64,
    successful_vote_transactions    UInt64,
    successful_non_vote_transactions UInt64,
    failed_vote_transactions        UInt64,
    failed_non_vote_transactions    UInt64
)
    ENGINE = ReplacingMergeTree()
    PRIMARY KEY (hash)
    ORDER BY (hash)
    COMMENT 'Solana block header';

CREATE TABLE IF NOT EXISTS rewards
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_date                  Date,
    block_hash                  String,

    -- block --
    block_slot                  UInt64,
    block_height                UInt64,
    block_previous_block_hash   String,
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

CREATE TABLE IF NOT EXISTS transactions
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_hash                  String,
    block_date                  Date,

     -- block --
    block_slot                  UInt64,
    block_height                UInt64,
    block_previous_block_hash   String,
    block_parent_slot           UInt64,

    -- transaction --
    id                          String,
    `index`                     UInt32,
    fee                         UInt64,
    required_signatures         UInt32,
    required_signed_accounts    UInt32,
    required_unsigned_accounts  UInt32,
    `signature`                 String,
    success                     Bool,
    error                       String,
    recent_block_hash           String,
    account_keys                String,
    log_messages                String,
    pre_balances                String,
    post_balances                String,
)

    ENGINE = ReplacingMergeTree()
    PRIMARY KEY (block_hash, id)
    ORDER BY (block_hash, id)
    COMMENT 'Solana transactions';

CREATE TABLE IF NOT EXISTS transaction_instructions
(
    -- clock --
    block_time                DateTime64(3, 'UTC'),
    block_hash                String,
    block_date                Date,

    -- block --
    block_slot                UInt64,
    block_height              UInt64,
    block_previous_block_hash String,
    block_parent_slot         UInt64,

    -- transaction --
    transaction_id            String,
    transaction_index         UInt32,

    -- instruction --
    instruction_index         UInt32,
    `data`                    String,
    executing_account         String,
    account_arguments         String
)

    ENGINE = ReplacingMergeTree()
    PRIMARY KEY (block_hash, transaction_id, instruction_index)
    ORDER BY (block_hash, transaction_id, instruction_index)
    COMMENT 'Solana transaction instructions';

CREATE TABLE IF NOT EXISTS transaction_inner_instructions
(
    -- clock --
    block_time                DateTime64(3, 'UTC'),
    block_height              UInt64,
    block_hash                String,
    block_date                Date,

    -- block --
    block_slot                UInt64,
    block_previous_block_hash String,
    block_parent_slot         UInt64,

    -- reference to transaction --
    transaction_id            String,
    instruction_index         UInt32, -- links to transaction_instructions

    -- inner instruction details --
    inner_instruction_index   UInt32,
    inner_data                String,
    inner_executing_account   String,
    inner_account_arguments   String
)

    ENGINE = ReplacingMergeTree()
    PRIMARY KEY (block_hash, transaction_id, instruction_index, inner_instruction_index)
    ORDER BY (block_hash, transaction_id, instruction_index, inner_instruction_index)
    COMMENT 'Solana transaction inner instructions';

CREATE TABLE IF NOT EXISTS token_balances
(
    -- clock --
    block_time                DateTime64(3, 'UTC'),
    block_height              UInt64,
    block_hash                String,
    block_date                Date,
    
    -- block --
    block_slot                UInt64,
    block_previous_block_hash String,
    block_parent_slot         UInt64,

    -- transaction --
    transaction_id            String,
    transaction_index         UInt32,

    -- token balance --
    program_id                String,
    account                   String,
    mint                      String,
    `owner`                   String,
    pre_amount                Decimal(38,18),
    post_amount               Decimal(38,18),
)

    ENGINE = ReplacingMergeTree()
    PRIMARY KEY (block_hash, transaction_id, program_id, account)
    ORDER BY (block_hash, transaction_id, program_id, account)
    COMMENT 'Solana token balances';

-- Projections --
-- https://clickhouse.com/docs/en/sql-reference/statements/alter/projection --
ALTER TABLE blocks ADD PROJECTION IF NOT EXISTS blocks_by_block_height (
    SELECT * ORDER BY date, height
);

ALTER TABLE rewards ADD PROJECTION IF NOT EXISTS rewards_by_block_height (
    SELECT * ORDER BY block_date, block_height
);

ALTER TABLE transactions ADD PROJECTION IF NOT EXISTS transactions_by_block_height (
    SELECT * ORDER BY block_date, block_height
);

ALTER TABLE transaction_instructions ADD PROJECTION IF NOT EXISTS transaction_instructions_by_block_height (
    SELECT * ORDER BY block_date, block_height
);

ALTER TABLE transaction_inner_instructions ADD PROJECTION IF NOT EXISTS transaction_inner_instructions_by_block_height (
    SELECT * ORDER BY block_date, block_height
);

ALTER TABLE token_balances ADD PROJECTION IF NOT EXISTS token_balances_by_block_height (
    SELECT * ORDER BY block_date, block_height
);


ALTER TABLE blocks MATERIALIZE PROJECTION blocks_by_block_height;

ALTER TABLE rewards MATERIALIZE PROJECTION rewards_by_block_height;

ALTER TABLE transactions MATERIALIZE PROJECTION transactions_by_block_height;

ALTER TABLE transaction_instructions MATERIALIZE PROJECTION transaction_instructions_by_block_height;

ALTER TABLE transaction_inner_instructions MATERIALIZE PROJECTION transaction_inner_instructions_by_block_height;

ALTER TABLE token_balances MATERIALIZE PROJECTION token_balances_by_block_height;
