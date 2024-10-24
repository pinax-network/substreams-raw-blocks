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

CREATE TABLE IF NOT EXISTS blocks 
(
    -- clock --
    time                        DateTime64(3, 'UTC'),
    number                      UInt64,
    date                        Date,
    hash                        String COMMENT 'Cosmos Hash',

    -- header --
    version_consensus_block     UInt64,
    version_consensus_app       UInt64,
    chain_id                    String,
    last_block_id               String,
    last_commit_hash            String,
    data_hash                   String,
    validators_hash             String,
    next_validators_hash        String,
    consensus_hash              String,
    app_hash                    String,
    last_results_hash           String,
    evidence_hash               String,
    proposer_address            String,

    -- counters --
    total_transactions          UInt64,
    successful_transactions     UInt64,
    failed_transactions         UInt64,
)

ENGINE = ReplacingMergeTree()
PRIMARY KEY (number)
ORDER BY (number)
COMMENT 'Cosmos block header';

CREATE TABLE IF NOT EXISTS transactions (
    -- clock --
    block_time                       DateTime64(3, 'UTC'),
    block_number                     UInt64,
    block_date                       Date,
    block_hash                       String COMMENT 'Cosmos Hash',

    -- transaction --
    `index`                           UInt32 COMMENT 'Transaction index in block',
    `hash`                            String COMMENT 'Transaction hash',
    code                             UInt32,
    `data`                           String,
    `log`                            String,
    info                             String,
    gas_wanted                       Int64,
    gas_used                         Int64,
    codespace                        String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (hash)
        ORDER BY (hash)
        COMMENT 'Cosmos transaction';

ALTER TABLE blocks ADD PROJECTION IF NOT EXISTS blocks_by_block_height (
    SELECT * ORDER BY date, number
);

ALTER TABLE transactions ADD PROJECTION IF NOT EXISTS transactions_by_hash (
    SELECT * ORDER BY block_date, block_number
);

ALTER TABLE blocks MATERIALIZE PROJECTION blocks_by_block_height;