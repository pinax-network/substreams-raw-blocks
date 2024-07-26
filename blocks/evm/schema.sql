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
    time                    DateTime,
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
    miner                   String,
    difficulty              Int64, -- Nullable
    total_difficulty        Int64, -- Nullable
    size                    String,
    mix_hash                String,
    extra_data              String,
    gas_limit               UInt64,
    gas_used                UInt64,
    blob_gas_used           UInt64, -- Nullable
    total_transactions      UInt64,
    successful_transactions UInt64,
    failed_transactions     UInt64,
    base_fee_per_gas        String, -- Nullable
    parent_beacon_root      String  -- Nullable
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (date, time, number, hash)
        ORDER BY (date, time, number, hash);

CREATE TABLE IF NOT EXISTS logs
(
    block_time          DateTime,
    block_number        UInt64,
    block_hash          String,
    block_date          Date,
    contract_address    String,
    topic0              String,
    topic1              String,
    topic2              String,
    topic3              String,
    data                String,
    log_index           UInt32,
    tx_hash             String,
    tx_index            UInt32,
    tx_from             String,
    tx_to               String

)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, log_index)
        ORDER BY (tx_hash, log_index);
