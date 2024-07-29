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
    date                    Date,
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
    block_date          Date,
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
    block_date          Date,
    address             String,
    new_value           UInt256,
    old_value           UInt256,
    ordinal             UInt64,
    reason              LowCardinality(String),
    reason_code         Int32,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_time, block_number, ordinal)
        ORDER BY (block_date, block_time, block_number, ordinal)
        COMMENT 'Ethereum balance changes';

CREATE TABLE IF NOT EXISTS traces
(
    -- block --
    block_time          DateTime('UTC'),
    block_number        UInt64,
    block_hash          String,
    block_date          Date,

    -- transaction --
    tx_hash             String,
    tx_index            UInt32,
    tx_status           LowCardinality(String),
    tx_status_code      Int32,
    tx_is_successful    Bool,
    from                String,
    to                  String,

    -- trace --
    index               UInt32,
    parent_index        UInt32,
    depth               UInt32,
    caller              String,
    call_type           LowCardinality(String),
    call_type_code      Int32,
    address             String,
    value               UInt256,
    gas_limit           UInt64,
    gas_consumed        UInt64,
    return_data         String,
    input               String,
    suicide             Bool,
    failure_reason      LowCardinality(String),
    state_reverted      Bool,
    status_reverted     Bool,
    status_failed       Bool,
    executed_code       Bool,
    begin_ordinal       UInt64,
    end_ordinal         UInt64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_time, block_number, tx_hash, tx_index, index)
        ORDER BY (block_date, block_time, block_number, tx_hash, tx_index, index)
        COMMENT 'Ethereum traces';
