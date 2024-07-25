
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
