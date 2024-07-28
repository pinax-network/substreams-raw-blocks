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

CREATE TABLE IF NOT EXISTS blocks
(
    time                    DateTime('UTC'),
    number                  UInt64,
    date                    LowCardinality(String),
    hash                    String,
    parent_hash             String,
    nonce                   UInt64,
    ommers_hash             String,
    logs_bloom              String,
    transactions_root       String,
    state_root              String,
    receipts_root           String,
    withdrawals_root        String DEFAULT '' COMMENT 'EIP-4895 (Shangai Fork)',
    parent_beacon_root      String DEFAULT '' COMMENT 'EIP-4788 (Dencun Fork)',
    miner                   String,
    difficulty              Int64 DEFAULT 0,
    total_difficulty        Int128 DEFAULT 0,
    size                    String,
    mix_hash                String,
    extra_data              String,
    gas_limit               UInt64,
    gas_used                UInt64,
    base_fee_per_gas        Int64 DEFAULT 0 COMMENT 'EIP-1559 (London Fork)',
    blob_gas_used           UInt64 DEFAULT 0 COMMENT 'EIP-4844 (Dencun Fork)',
    excess_blob_gas         UInt64 DEFAULT 0 COMMENT 'EIP-4844 (Dencun Fork)',
    total_transactions      UInt64,
    successful_transactions UInt64,
    failed_transactions     UInt64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (date, time, number, hash)
        ORDER BY (date, time, number, hash)
        COMMENT 'Ethereum block header';

CREATE TABLE IF NOT EXISTS logs
(
    block_time          DateTime('UTC'),
    block_number        UInt64,
    block_hash          String,
    block_date          LowCardinality(String),
    contract_address    String,
    topic0              String,
    topic1              String DEFAULT '',
    topic2              String DEFAULT '',
    topic3              String DEFAULT '',
    data                String,
    log_index           UInt32,
    tx_hash             String,
    tx_index            UInt32,
    tx_from             String,
    tx_to               String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_time, block_number, log_index, tx_hash)
        ORDER BY (block_date, block_time, block_number, log_index, tx_hash)
        COMMENT 'Ethereum event logs';

CREATE TABLE IF NOT EXISTS balance_changes
(
    block_time          DateTime('UTC'),
    block_number        UInt64,
    block_hash          String,
    block_date          LowCardinality(String),
    address             String,
    new_value           String,
    old_value           String,
    ordinal             UInt64,
    reason              Int32
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_time, block_number, log_index, tx_hash)
        ORDER BY (block_date, block_time, block_number, log_index, tx_hash)
        COMMENT 'Ethereum balance changes';