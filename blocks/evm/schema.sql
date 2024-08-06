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
    difficulty              UInt64 DEFAULT 0,
    total_difficulty        String DEFAULT '' COMMENT 'UInt256',
    size                    String,
    mix_hash                String,
    extra_data              String,
    extra_data_utf8         String,
    gas_limit               UInt64,
    gas_used                UInt64,
    base_fee_per_gas        String DEFAULT '' COMMENT 'EIP-1559 (London Fork)',
    blob_gas_used           String DEFAULT '' COMMENT 'EIP-4844 (Dencun Fork)',
    excess_blob_gas         String DEFAULT '' COMMENT 'EIP-4844 (Dencun Fork)',
    total_transactions      UInt64,
    successful_transactions UInt64,
    failed_transactions     UInt64,
    total_balance_changes   UInt64,
    total_withdrawals       UInt64

)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (date, time, number, hash)
        ORDER BY (date, time, number, hash)
        COMMENT 'EVM block header';

CREATE TABLE IF NOT EXISTS logs
(
    -- block --
    block_time          DateTime('UTC'),
    block_number        UInt64,
    block_hash          String,
    block_date          Date,

    -- transaction --
    tx_hash                     String,
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    from                        String COMMENT 'EVM Address',
    to                          String COMMENT 'EVM Address',

    -- logs --
    `index`             UInt32,
    contract_address    String,
    topic0              String,
    topic1              String DEFAULT '',
    topic2              String DEFAULT '',
    topic3              String DEFAULT '',
    data                String

)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_time, block_number, tx_hash, `index`)
        ORDER BY (block_date, block_time, block_number, tx_hash, `index`)
        COMMENT 'EVM event logs';

CREATE TABLE IF NOT EXISTS block_balance_changes
(
    -- block --
    block_time                  DateTime('UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- balance change --
    address                     String,
    new_value                   DEFAULT '' COMMENT 'UInt256',
    old_value                   DEFAULT '' COMMENT 'UInt256',
    delta_value                 DEFAULT '' COMMENT 'Int256',
    ordinal                     UInt64,
    reason                      LowCardinality(String),
    reason_code                 UInt32,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_time, block_number, ordinal)
        ORDER BY (block_date, block_time, block_number, ordinal)
        COMMENT 'EVM block balance changes';

CREATE TABLE IF NOT EXISTS balance_changes
(
    -- block --
    block_time                  DateTime('UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- transaction --
    tx_hash                     String,
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    from                        String COMMENT 'EVM Address',
    to                          String COMMENT 'EVM Address',

    -- trace --
    trace_index                 UInt32,
    trace_parent_index          UInt32,
    trace_depth                 UInt32,
    trace_caller                String,

    -- balance change --
    address                     String,
    new_value                   DEFAULT '' COMMENT 'UInt256',
    old_value                   DEFAULT '' COMMENT 'UInt256',
    delta_value                 DEFAULT '' COMMENT 'Int256',
    ordinal                     UInt64,
    reason                      LowCardinality(String),
    reason_code                 UInt32,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_time, block_number, ordinal)
        ORDER BY (block_date, block_time, block_number, ordinal)
        COMMENT 'EVM balance changes';

CREATE TABLE IF NOT EXISTS traces
(
    -- block --
    block_time                  DateTime('UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- transaction --
    tx_hash                     String,
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
    caller                      String,
    call_type                   LowCardinality(String),
    call_type_code              UInt32,
    address                     String,
    value                       DEFAULT '' COMMENT 'UInt256',
    gas_limit                   UInt64,
    gas_consumed                UInt64,
    return_data                 String,
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
        PRIMARY KEY (block_date, block_time, block_number, tx_hash, tx_index, `index`)
        ORDER BY (block_date, block_time, block_number, tx_hash, tx_index, `index`)
        COMMENT 'EVM traces';


CREATE TABLE IF NOT EXISTS transactions
(
    -- block --
    block_time                  DateTime('UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- transaction --
    `index`                     UInt32,
    hash                        String,
    from                        String,
    to                          String,
    nonce                       UInt64,
    status                      LowCardinality(String),
    status_code                 UInt32,
    success                     Bool,
    gas_price                   DEFAULT '' COMMENT 'UInt256',
    gas_limit                   UInt64,
    value                       DEFAULT '' COMMENT 'UInt256',
    input                       String,
    v                           String,
    r                           String,
    s                           String,
    gas_used                    UInt64,
    type                        LowCardinality(String),
    type_code                   UInt32,
    max_fee_per_gas             DEFAULT '' COMMENT 'UInt256',
    max_priority_fee_per_gas    DEFAULT '' COMMENT 'UInt256',
    return_data                 String,
    public_key                  String,
    begin_ordinal               UInt64,
    end_ordinal                 UInt64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_time, block_number, hash)
        ORDER BY (block_date, block_time, block_number, hash)
        COMMENT 'EVM transactions';

CREATE TABLE IF NOT EXISTS storage_changes
(
    -- block --
    block_time                  DateTime('UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- transaction --
    tx_hash                     String,
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    from                        String COMMENT 'EVM Address',
    to                          String COMMENT 'EVM Address',

    -- trace --
    trace_index                 UInt32,
    trace_parent_index          UInt32,
    trace_depth                 UInt32,
    trace_caller                String,

    -- storage change --
    address                     String,
    key                         String,
    new_value                   String DEFAULT '',
    old_value                   String DEFAULT '',
    ordinal                     UInt64,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_time, block_number, ordinal)
        ORDER BY (block_date, block_time, block_number, ordinal)
        COMMENT 'EVM storage changes';

CREATE TABLE IF NOT EXISTS code_changes
(
    -- block --
    block_time                  DateTime('UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- transaction --
    tx_hash                     String,
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    from                        String COMMENT 'EVM Address',
    to                          String COMMENT 'EVM Address',

    -- trace --
    trace_index                 UInt32,
    trace_parent_index          UInt32,
    trace_depth                 UInt32,
    trace_caller                String,

    -- code change --
    address                     String,
    old_hash                    String DEFAULT '',
    old_code                    String DEFAULT '',
    new_hash                    String DEFAULT '',
    new_code                    String DEFAULT '',
    ordinal                     UInt64,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_time, block_number, ordinal)
        ORDER BY (block_date, block_time, block_number, ordinal)
        COMMENT 'EVM code changes';

CREATE TABLE IF NOT EXISTS account_creations
(
    -- block --
    block_time                  DateTime('UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- transaction --
    tx_hash                     String,
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    from                        String COMMENT 'EVM Address',
    to                          String COMMENT 'EVM Address',

    -- trace --
    trace_index                 UInt32,
    trace_parent_index          UInt32,
    trace_depth                 UInt32,
    trace_caller                String,

    -- account creation --
    account                     String,
    ordinal                     UInt64,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_time, block_number, ordinal)
        ORDER BY (block_date, block_time, block_number, ordinal)
        COMMENT 'EVM account creations';

CREATE TABLE IF NOT EXISTS nonce_changes
(
    -- block --
    block_time                  DateTime('UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- transaction --
    tx_hash                     String,
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    from                        String COMMENT 'EVM Address',
    to                          String COMMENT 'EVM Address',

    -- trace --
    trace_index                 UInt32,
    trace_parent_index          UInt32,
    trace_depth                 UInt32,
    trace_caller                String,

    -- nonce change --
    address                     String,
    old_value                   UInt64,
    new_value                   UInt64,
    ordinal                     UInt64,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_time, block_number, ordinal)
        ORDER BY (block_date, block_time, block_number, ordinal)
        COMMENT 'EVM nonce changes';

CREATE TABLE IF NOT EXISTS gas_changes
(
    -- block --
    block_time                  DateTime('UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- transaction --
    tx_hash                     String,
    tx_index                    UInt32,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt32,
    tx_success                  Bool,
    from                        String COMMENT 'EVM Address',
    to                          String COMMENT 'EVM Address',

    -- trace --
    trace_index                 UInt32,
    trace_parent_index          UInt32,
    trace_depth                 UInt32,
    trace_caller                String,

    -- gas change --
    old_value                   UInt64,
    new_value                   UInt64,
    delta_value                 DEFAULT '' COMMENT 'Int128',
    reason                      LowCardinality(String),
    reason_code                 UInt32,
    ordinal                     UInt64,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_time, block_number, ordinal)
        ORDER BY (block_date, block_time, block_number, ordinal)
        COMMENT 'EVM gas changes';