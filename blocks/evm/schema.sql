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
    hash                        FixedString(66),

    -- header --
    parent_hash                 FixedString(66),
    nonce                       UInt64,
    ommers_hash                 FixedString(66),
    logs_bloom                  String,
    transactions_root           FixedString(66),
    state_root                  FixedString(66),
    receipts_root               FixedString(66),
    withdrawals_root            FixedString(66) DEFAULT '' COMMENT 'EIP-4895 (Shangai Fork)',
    parent_beacon_root          FixedString(66) DEFAULT '' COMMENT 'EIP-4788 (Dencun Fork)',
    miner                       FixedString(42),
    difficulty                  UInt64 DEFAULT 0,
    total_difficulty            String DEFAULT '' COMMENT 'UInt256',
    size                        String,
    mix_hash                    FixedString(66),
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
    block_hash                  FixedString(66),
    block_date                  Date,

    -- block roots --
    transactions_root           FixedString(66),
    receipts_root               FixedString(66),

    -- transaction --
    `index`                     UInt32,
    hash                        FixedString(66),
    from                        FixedString(42),
    to                          FixedString(42),
    nonce                       UInt64,
    status                      LowCardinality(String),
    status_code                 UInt32,
    success                     Bool,
    gas_price                   DEFAULT '' COMMENT 'UInt256',
    gas_limit                   UInt64,
    value                       DEFAULT '' COMMENT 'UInt256',
    input                       String,
    v                           String,
    r                           FixedString(66),
    s                           FixedString(66),
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
    state_root                  FixedString(66),
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
    block_hash                  FixedString(66),
    block_date                  Date,

    -- transaction --
    tx_hash                     FixedString(66),
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    tx_from                     FixedString(42) COMMENT 'EVM Address',
    tx_to                       FixedString(42) COMMENT 'EVM Address',

    -- logs --
    `index`                     UInt32,
    block_index                 UInt32,
    contract_address            FixedString(42),
    topic0                      FixedString(66),
    topic1                      FixedString(66) DEFAULT '',
    topic2                      FixedString(66) DEFAULT '',
    topic3                      FixedString(66) DEFAULT '',
    data                        String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, tx_hash, `index`)
        COMMENT 'EVM event logs';

CREATE TABLE IF NOT EXISTS block_balance_changes
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  FixedString(66),
    block_date                  Date,

    -- balance change --
    address                     FixedString(42),
    new_value                   DEFAULT '' COMMENT 'UInt256',
    old_value                   DEFAULT '' COMMENT 'UInt256',
    delta_value                 DEFAULT '' COMMENT 'Int256',
    ordinal                     UInt64,
    reason                      LowCardinality(String),
    reason_code                 UInt32
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, block_hash, ordinal)
        COMMENT 'EVM block balance changes';

CREATE TABLE IF NOT EXISTS balance_changes
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  FixedString(66),
    block_date                  Date,

    -- transaction --
    tx_hash                     FixedString(66),
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    tx_from                     FixedString(42) COMMENT 'EVM Address',
    tx_to                       FixedString(42) COMMENT 'EVM Address',

    -- trace --
    trace_index                 UInt32,
    trace_parent_index          UInt32,
    trace_depth                 UInt32,
    trace_caller                FixedString(42),

    -- balance change --
    address                     FixedString(42),
    new_value                   DEFAULT '' COMMENT 'UInt256',
    old_value                   DEFAULT '' COMMENT 'UInt256',
    delta_value                 DEFAULT '' COMMENT 'Int256',
    ordinal                     UInt64,
    reason                      LowCardinality(String),
    reason_code                 UInt32
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, block_hash, ordinal)
        COMMENT 'EVM balance changes';

CREATE TABLE IF NOT EXISTS traces
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  FixedString(66),
    block_date                  Date,

    -- transaction --
    tx_hash                     FixedString(66),
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    from                        FixedString(42) COMMENT 'EVM Address',
    to                          FixedString(42) COMMENT 'EVM Address',

    -- trace --
    `index`                     UInt32,
    parent_index                UInt32,
    depth                       UInt32,
    caller                      FixedString(42),
    call_type                   LowCardinality(String),
    call_type_code              UInt32,
    address                     FixedString(42),
    value                       DEFAULT '' COMMENT 'UInt256',
    gas_limit                   UInt64,
    gas_consumed                UInt64,
    return_data                 String COMMENT 'Return data is set by contract calls using RETURN or REVERT.',
    input                       String,
    method_id                   String(8) COMMENT 'Method ID is the first 4 bytes of the Keccak-256 hash of the function signature.'
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



CREATE TABLE IF NOT EXISTS storage_changes
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  FixedString(66),
    block_date                  Date,

    -- transaction --
    tx_hash                     FixedString(66),
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    tx_from                     FixedString(42) COMMENT 'EVM Address',
    tx_to                       FixedString(42) COMMENT 'EVM Address',

    -- trace --
    trace_index                 UInt32,
    trace_parent_index          UInt32,
    trace_depth                 UInt32,
    trace_caller                FixedString(42),

    -- storage change --
    address                     FixedString(42),
    key                         FixedString(66),
    new_value                   FixedString(66) DEFAULT '',
    old_value                   FixedString(66) DEFAULT '',
    ordinal                     UInt64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, block_hash, ordinal)
        COMMENT 'EVM storage changes';

CREATE TABLE IF NOT EXISTS block_code_changes
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  FixedString(66),
    block_date                  Date,

    -- code change --
    address                     FixedString(42),
    old_hash                    FixedString(66) DEFAULT '',
    old_code                    String DEFAULT '',
    new_hash                    FixedString(66) DEFAULT '',
    new_code                    String DEFAULT '',
    ordinal                     UInt64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, block_hash, ordinal)
        COMMENT 'EVM block code changes';

CREATE TABLE IF NOT EXISTS code_changes
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  FixedString(66),
    block_date                  Date,

    -- transaction --
    tx_hash                     FixedString(66),
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    tx_from                        FixedString(42) COMMENT 'EVM Address',
    tx_to                          FixedString(42) COMMENT 'EVM Address',

    -- trace --
    trace_index                 UInt32,
    trace_parent_index          UInt32,
    trace_depth                 UInt32,
    trace_caller                FixedString(42),

    -- code change --
    address                     FixedString(42),
    old_hash                    FixedString(66) DEFAULT '',
    old_code                    String DEFAULT '',
    new_hash                    FixedString(66) DEFAULT '',
    new_code                    String DEFAULT '',
    ordinal                     UInt64
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
    block_hash                  FixedString(66),
    block_date                  Date,

    -- transaction --
    tx_hash                     FixedString(66),
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    tx_from                     FixedString(42) COMMENT 'EVM Address',
    tx_to                       FixedString(42) COMMENT 'EVM Address',

    -- trace --
    trace_index                 UInt32,
    trace_parent_index          UInt32,
    trace_depth                 UInt32,
    trace_caller                FixedString(42),

    -- account creation --
    account                     FixedString(42),
    ordinal                     UInt64
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
    block_hash                  FixedString(66),
    block_date                  Date,

    -- transaction --
    tx_hash                     FixedString(66),
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    tx_from                     FixedString(42) COMMENT 'EVM Address',
    tx_to                       FixedString(42) COMMENT 'EVM Address',

    -- trace --
    trace_index                 UInt32,
    trace_parent_index          UInt32,
    trace_depth                 UInt32,
    trace_caller                FixedString(42),

    -- nonce change --
    address                     FixedString(42),
    old_value                   UInt64,
    new_value                   UInt64,
    ordinal                     UInt64
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
    block_hash                  FixedString(66),
    block_date                  Date,

    -- transaction --
    tx_hash                     FixedString(66),
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    tx_from                     FixedString(42) COMMENT 'EVM Address',
    tx_to                       FixedString(42) COMMENT 'EVM Address',

    -- trace --
    trace_index                 UInt32,
    trace_parent_index          UInt32,
    trace_depth                 UInt32,
    trace_caller                FixedString(42),

    -- gas change --
    old_value                   UInt64,
    new_value                   UInt64,
    delta_value                 String DEFAULT '' COMMENT 'Int128',
    reason                      LowCardinality(String),
    reason_code                 UInt32,
    ordinal                     UInt64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, block_hash, ordinal)
        COMMENT 'EVM gas changes';

CREATE TABLE IF NOT EXISTS system_traces
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  FixedString(66),
    block_date                  Date,

    -- trace --
    `index`                     UInt32,
    parent_index                UInt32,
    depth                       UInt32,
    caller                      FixedString(42),
    call_type                   LowCardinality(String),
    call_type_code              UInt32,
    address                     FixedString(42),
    value                       DEFAULT '' COMMENT 'UInt256',
    gas_limit                   UInt64,
    gas_consumed                UInt64,
    return_data                 String COMMENT 'Return data is set by contract calls using RETURN or REVERT.',
    input                       String,
    suicide                     Bool,
    failure_reason              LowCardinality(String),
    state_reverted              Bool,
    status_reverted             Bool,
    status_failed               Bool,
    executed_code               Bool,
    begin_ordinal               UInt64,
    end_ordinal                 UInt64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, `index`)
        COMMENT 'EVM system traces';