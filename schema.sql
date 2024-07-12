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

CREATE TABLE IF NOT EXISTS RAW_DATA_BLOCKS
(
    timestamp               DateTime,
    number                  UInt64,
    hash                    String,
    parent_hash             String,
    date                    Date,
    nonce                   UInt64,
    sha3_uncles             String,
    logs_bloom              String,
    transactions_root       String,
    state_root              String,
    receipts_root           String,
    miner                   String,
    difficulty              Int64,
    total_difficulty        Decimal(38, 0),
    size                    String,
    mix_hash                String,
    extra_data              String,
    gas_limit               UInt64,
    gas_used                UInt64,
    blob_gas_used           UInt64,
    transaction_count       String,
    base_fee_per_gas        String,
    parent_beacon_root      String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (hash)
        ORDER BY (hash);

CREATE TABLE IF NOT EXISTS logs
(
    block_hash  String,
    block_num   UInt64,
    timestamp   DateTime,
    tx_hash     String,
    tx_index    UInt32,
    log_index   UInt32,
    address     String,
    topic0      String,
    topic1      String,
    topic2      String,
    topic3      String,
    data        String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, log_index)
        ORDER BY (tx_hash, log_index);

CREATE TABLE IF NOT EXISTS calls
(
    block_hash      String,
    block_num       UInt64,
    timestamp       DateTime,
    tx_index        UInt32,
    tx_hash         String,
    index           UInt32,
    parent_index    UInt32,
    depth           UInt32,
    caller          String,
    address         String,
    value           Float64,
    gas_limit       UInt64,
    gas_consumed    UInt64,
    return_data     String,
    input           String,
    selfdestruct    UInt8,
    executed_code   UInt8,
    begin_ordinal   UInt64,
    end_ordinal     UInt64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, index)
        ORDER BY (tx_hash, index);

CREATE TABLE IF NOT EXISTS transactions
(
    block_hash              String,
    block_num               UInt64,
    timestamp               DateTime,
    tx_index                UInt32,
    tx_hash                 String,
    to                      String,
    nonce                   UInt64,
    gas_price               Float64,
    gas_limit               UInt64,
    value                   Float64,
    input                   String,
    v                       String,
    r                       String,
    s                       String,
    gas_used                UInt64,
    type                    Int32,
    max_fee_per_gas         Float64,
    max_priority_fee_per_gas Float64,
    from                    String,
    return_data             String,
    public_key              String,
    begin_ordinal           UInt64,
    end_ordinal             UInt64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash)
        ORDER BY (tx_hash);

-- trace_type
-- status
