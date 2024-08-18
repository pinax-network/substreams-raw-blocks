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
    -- clock --
    time                        DateTime64(3, 'UTC'),
    number                      UInt64,
    date                        Date,
    hash                        String COMMENT 'EVM Hash',

    -- header --
    parent_hash                 String COMMENT 'EVM Hash',
    nonce                       UInt64,
    ommers_hash                 String COMMENT 'EVM Hash',
    logs_bloom                  String,
    transactions_root           String COMMENT 'EVM Hash',
    state_root                  String COMMENT 'EVM Hash',
    receipts_root               String COMMENT 'EVM Hash',
    withdrawals_root            String DEFAULT '' COMMENT 'EVM Root EIP-4895 (Shangai Fork)',
    parent_beacon_root          String DEFAULT '' COMMENT 'EVM Root EIP-4788 (Dencun Fork)',
    miner                       String COMMENT 'EVM Address',
    difficulty                  UInt64 DEFAULT 0,
    total_difficulty            String DEFAULT '' COMMENT 'UInt256',
    size                        String,
    mix_hash                    String COMMENT 'EVM Hash',
    extra_data                  String,
    extra_data_utf8             String,
    gas_limit                   UInt64,
    gas_used                    UInt64,
    base_fee_per_gas            String DEFAULT '' COMMENT 'EIP-1559 (London Fork)',
    blob_gas_used               String DEFAULT '' COMMENT 'EIP-4844 (Dencun Fork)',
    excess_blob_gas             String DEFAULT '' COMMENT 'EIP-4844 (Dencun Fork)',
    total_transactions          UInt64,
    successful_transactions     UInt64,
    failed_transactions         UInt64,
    total_balance_changes       UInt64,
    total_withdrawals           UInt64,

    -- detail level --
    detail_level                LowCardinality(String),
    detail_level_code           UInt32,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (date, number)
        ORDER BY (date, number, hash)
        COMMENT 'EVM block header';


CREATE TABLE IF NOT EXISTS transactions
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'EVM Hash',
    block_date                  Date,

    -- block roots --
    transactions_root           String COMMENT 'EVM Hash',
    receipts_root               String COMMENT 'EVM Hash',

    -- transaction --
    `index`                     UInt32,
    hash                        String COMMENT 'EVM Hash',
    from                        String COMMENT 'EVM Address',
    to                          String COMMENT 'EVM Address',
    nonce                       UInt64,
    status                      LowCardinality(String),
    status_code                 UInt32,
    success                     Bool,
    gas_price                   DEFAULT '' COMMENT 'UInt256',
    gas_limit                   UInt64,
    value                       DEFAULT '' COMMENT 'UInt256',
    data                        String,
    method_id                   LowCardinality(String) COMMENT 'Method ID is the first 4 bytes of the Keccak-256 hash of the function signature.',
    v                           String,
    r                           String COMMENT 'EVM Hash',
    s                           String COMMENT 'EVM Hash',
    gas_used                    UInt64,
    type                        LowCardinality(String) COMMENT 'EIP-1559',
    type_code                   UInt32 COMMENT 'EIP-1559',
    max_fee_per_gas             DEFAULT '' COMMENT 'UInt256',
    max_priority_fee_per_gas    DEFAULT '' COMMENT 'UInt256',
    begin_ordinal               UInt64,
    end_ordinal                 UInt64,

    -- transaction receipt --
    blob_gas_price              DEFAULT '' COMMENT 'UInt256',
    blob_gas_used               UInt64,
    cumulative_gas_used         UInt64,
    logs_bloom                  String,
    state_root                  String COMMENT 'EVM Hash',
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, hash)
        COMMENT 'EVM transactions';

CREATE TABLE IF NOT EXISTS logs
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'EVM Hash',
    block_date                  Date,

    -- transaction --
    tx_hash                     String COMMENT 'EVM Hash',
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    tx_from                     String COMMENT 'EVM Address',
    tx_to                       String COMMENT 'EVM Address',

    -- logs --
    `index`                     UInt32,
    block_index                 UInt32,
    contract_address            String COMMENT 'EVM Address',
    topic0                      String COMMENT 'EVM Hash',
    topic1                      String DEFAULT '' COMMENT 'EVM Hash',
    topic2                      String DEFAULT '' COMMENT 'EVM Hash',
    topic3                      String DEFAULT '' COMMENT 'EVM Hash',
    data                        String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, tx_hash, `index`)
        COMMENT 'EVM event logs';

CREATE TABLE IF NOT EXISTS traces
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'EVM Hash',
    block_date                  Date,

    -- transaction --
    tx_hash                     String COMMENT 'EVM Hash',
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    from                        String COMMENT 'EVM Address',
    to                          String COMMENT 'EVM Address',

    -- trace --
    `index`                     UInt32,
    parent_index                UInt32,
    depth                       UInt32,
    caller                      String COMMENT 'EVM Address',
    call_type                   LowCardinality(String),
    call_type_code              UInt32,
    address                     String COMMENT 'EVM Address',
    value                       DEFAULT '' COMMENT 'UInt256',
    gas_limit                   UInt64,
    gas_consumed                UInt64,
    return_data                 String COMMENT 'Return data is set by contract calls using RETURN or REVERT.',
    input                       String,
    method_id                   LowCardinality(String) COMMENT 'Method ID is the first 4 bytes of the Keccak-256 hash of the function signature.',
    suicide                     Bool,
    failure_reason              LowCardinality(String),
    state_reverted              Bool,
    status_reverted             Bool,
    status_failed               Bool,
    executed_code               Bool,
    begin_ordinal               UInt64,
    end_ordinal                 UInt64,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, tx_hash, tx_index, `index`)
        COMMENT 'EVM traces';

CREATE TABLE IF NOT EXISTS balance_changes
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'EVM Hash',
    block_date                  Date,

    -- balance change --
    address                     String COMMENT 'EVM Address',
    new_balance                 DEFAULT '' COMMENT 'UInt256',
    old_balance                 DEFAULT '' COMMENT 'UInt256',
    amount                      DEFAULT '' COMMENT 'Int256',
    ordinal                     UInt64,
    reason                      LowCardinality(String),
    reason_code                 UInt32
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, block_hash, ordinal)
        COMMENT 'EVM balance changes';

CREATE TABLE IF NOT EXISTS storage_changes
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'EVM Hash',
    block_date                  Date,

    -- storage change --
    ordinal                     UInt64 COMMENT 'Block global ordinal',
    address                     String COMMENT 'EVM Address',
    key                         String COMMENT 'EVM Hash',
    new_value                   String DEFAULT '' COMMENT 'EVM Hash',
    old_value                   String DEFAULT '' COMMENT 'EVM Hash',
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, block_hash, ordinal)
        COMMENT 'EVM storage changes';

CREATE TABLE IF NOT EXISTS code_changes
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'EVM Hash',
    block_date                  Date,

    -- code change --
    ordinal                     UInt64 COMMENT 'Block global ordinal',
    address                     String COMMENT 'EVM Address',
    old_hash                    String DEFAULT '' COMMENT 'EVM Hash',
    old_code                    String DEFAULT '',
    new_hash                    String DEFAULT '' COMMENT 'EVM Hash',
    new_code                    String DEFAULT '',
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, block_hash, ordinal)
        COMMENT 'EVM code changes';

CREATE TABLE IF NOT EXISTS account_creations
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'EVM Hash',
    block_date                  Date,

    -- account creation --
    ordinal                     UInt64 COMMENT 'Block global ordinal',
    account                     String COMMENT 'EVM Address',
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, block_hash, ordinal)
        COMMENT 'EVM account creations';

CREATE TABLE IF NOT EXISTS nonce_changes
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'EVM Hash',
    block_date                  Date,

    -- nonce change --
    ordinal                     UInt64 COMMENT 'Block global ordinal',
    address                     String COMMENT 'EVM Address',
    old_value                   UInt64,
    new_value                   UInt64,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, ordinal)
        COMMENT 'EVM nonce changes';

CREATE TABLE IF NOT EXISTS gas_changes
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'EVM Hash',
    block_date                  Date,

    -- gas change --
    ordinal                     UInt64 COMMENT 'Block global ordinal',
    old_value                   UInt64,
    new_value                   UInt64,
    reason                      LowCardinality(String),
    reason_code                 UInt32,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, block_hash, ordinal)
        COMMENT 'EVM gas changes';
