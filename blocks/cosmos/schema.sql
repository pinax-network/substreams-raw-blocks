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

ALTER TABLE blocks ADD PROJECTION IF NOT EXISTS blocks_by_block_height (
    SELECT * ORDER BY date, number
);

ALTER TABLE blocks MATERIALIZE PROJECTION blocks_by_block_height;