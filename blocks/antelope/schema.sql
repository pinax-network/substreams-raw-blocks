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
    time                                    DateTime64(3, 'UTC'),
    number                                  UInt64,
    date                                    Date,
    hash                                    String COMMENT 'Hash',

    -- header --
    parent_hash                             String COMMENT 'Hash',
    producer                                String COMMENT 'Address',
    confirmed                               UInt32,
    schedule_version                        UInt32,

    -- block --
    version                                 UInt32,
    producer_signature                      String COMMENT 'Signature',
    dpos_proposed_irreversible_blocknum     UInt32,
    dpos_irreversible_blocknum              UInt32,

    -- block roots --
    transaction_mroot                       String COMMENT 'Hash',
    action_mroot                            String COMMENT 'Hash',
    -- blockroot_merkle_active_nodes           Array(String) COMMENT 'A blockroot Merkle tree uses hashes to verify blockchain data integrity. Leaf nodes hash data blocks, non-leaf nodes hash child nodes. The root hash efficiently verifies all data.',
    blockroot_merkle_node_count             UInt32,

    -- counters --
    size                                    UInt64 COMMENT 'Block size estimate in bytes',
    total_transactions                      UInt64,
    successful_transactions                 UInt64,
    failed_transactions                     UInt64,
    total_actions                           UInt64,
    total_db_ops                            UInt64,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (date, number)
        ORDER BY (date, number, hash)
        COMMENT 'Antelope block header';

CREATE TABLE IF NOT EXISTS transactions
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'Hash',
    block_date                  Date,

    -- transaction --
    hash                        String COMMENT 'Hash',
    `index`                     UInt64,
    elapsed                     Int64,
    net_usage                   UInt64,
    scheduled                   Bool,

    -- header --
    cpu_usage_micro_seconds     UInt32,
    net_usage_words             UInt32,
    status                      LowCardinality(String) COMMENT 'Status',
    status_code                 UInt8,
    success                     Bool,

    -- block roots --
    transaction_mroot           String COMMENT 'Hash',
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, hash)
        COMMENT 'Antelope transactions';

CREATE TABLE IF NOT EXISTS feature_ops
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'Hash',
    block_date                  Date,

    -- transaction --
    tx_hash                     String COMMENT 'Hash',
    tx_index                    UInt64,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt8,
    tx_success                  Bool,

    -- feature op --
    kind                        LowCardinality(String),
    action_index                UInt32,
    feature_digest              String,
    -- feature --    
    description_digest          String,
    --dependencies              Array(String),
    protocol_feature_type       LowCardinality(String),
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, tx_hash, action_index)
        COMMENT 'Antelope feature operations';

CREATE TABLE IF NOT EXISTS perm_ops
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'Hash',
    block_date                  Date,

    -- transaction --
    tx_hash                     String COMMENT 'Hash',
    tx_index                    UInt64,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt8,
    tx_success                  Bool,

    -- perm_op --
    operation                   LowCardinality(String),
    operation_code              UInt8,
    action_index                UInt32,
    id                          UInt64,
    parent_id                   UInt64,
    owner                       String,
    name                        String,
    threshold                   UInt32,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, action_index)
        ORDER BY (tx_hash, action_index)
        COMMENT 'Antelope permission operations';

CREATE TABLE IF NOT EXISTS accounts
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'Hash',
    block_date                  Date,

    -- transaction --
    tx_hash                     String COMMENT 'Hash',
    tx_index                    UInt64,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt8,
    tx_success                  Bool,

    -- authority.accounts --
    `index`                     UInt32,
    action_index                UInt32,
    actor                       String,
    permission                  LowCardinality(String),
    weight                      UInt32,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, action_index)
        ORDER BY (tx_hash, action_index, `index`)
        COMMENT 'Antelope authority accounts';

CREATE TABLE IF NOT EXISTS keys
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'Hash',
    block_date                  Date,

    -- transaction --
    tx_hash                     String COMMENT 'Hash',
    tx_index                    UInt64,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt8,
    tx_success                  Bool,

    -- authority.keys --
    `index`                     UInt32,
    action_index                UInt32,
    public_key                  String,
    weight                      UInt32,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, action_index)
        ORDER BY (tx_hash, action_index, `index`)
        COMMENT 'Antelope authority keys';

CREATE TABLE IF NOT EXISTS waits
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'Hash',
    block_date                  Date,

    -- transaction --
    tx_hash                     String COMMENT 'Hash',
    tx_index                    UInt64,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt8,
    tx_success                  Bool,

    -- authority.waits --
    `index`                     UInt32,
    action_index                UInt32,
    wait_sec                    UInt32,
    weight                      UInt32,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, action_index)
        ORDER BY (tx_hash, action_index, `index`)
        COMMENT 'Antelope authority waits';

CREATE TABLE IF NOT EXISTS actions
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'Hash',
    block_date                  Date,

    -- transaction --
    tx_hash                     String COMMENT 'Hash',
    tx_index                    UInt64,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt8,
    tx_success                  Bool,

    -- receipt --
    abi_sequence                UInt64,
    code_sequence               UInt64,
    digest                      String,
    global_sequence             UInt64,
    receipt_receiver            String COMMENT 'Address',
    recv_sequence               UInt64,

    -- action --
    account                     String COMMENT 'Address',
    name                        String COMMENT 'Address',
    json_data                   String COMMENT 'JSON',
    raw_data                    String COMMENT 'Hex',

    -- trace --
    `index`                                         UInt32 COMMENT 'Execution Index',
    action_ordinal                                  UInt32 COMMENT 'Action Ordinal',
    receiver                                        String,
    context_free                                    Bool,
    elapsed                                         Int64,
    console                                         String,
    raw_return_value                                String,
    json_return_value                               String,
    creator_action_ordinal                          UInt32,
    closest_unnotified_ancestor_action_ordinal      UInt32,

    -- block roots --
    action_mroot                                    String,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, tx_hash, `index`)
        COMMENT 'Antelope actions';

CREATE TABLE IF NOT EXISTS db_ops
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- transaction --
    tx_hash                     String,
    tx_index                    UInt64,
    tx_status                   LowCardinality(String),
    tx_status_code              UInt8,
    tx_success                  Bool,

    -- database operation --
    `index`                     UInt32,
    operation                   LowCardinality(String) COMMENT 'Operation',
    operation_code              UInt8,
    action_index                UInt32,
    code                        String,
    scope                       String,
    table_name                  String,
    primary_key                 String,
    old_payer                   String,
    new_payer                   String,
    old_data                    String,
    new_data                    String,
    old_data_json               String,
    new_data_json               String,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, tx_hash, `index`)
        COMMENT 'Antelope database operations';

CREATE TABLE IF NOT EXISTS authorizations
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- transaction --
    tx_hash                     String,

    -- action --
    action_index                UInt32,

    -- authorization --
    actor                       String,
    permission                  LowCardinality(String)
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, tx_hash, action_index, actor, permission)
        COMMENT 'Antelope action authorizations';

CREATE TABLE IF NOT EXISTS auth_sequences
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- transaction --
    tx_hash                     String,

    -- action --
    action_index                UInt32,

    -- auth_sequence --
    account_name                String,
    sequence                    UInt64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, tx_hash, action_index, account_name, sequence)
        COMMENT 'Antelope action authorization sequences';

CREATE TABLE IF NOT EXISTS account_ram_deltas
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- transaction --
    tx_hash                     String,

    -- action --
    action_index                UInt32,

    -- account_ram_delta --
    account                     String,
    delta                       Int64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, tx_hash, action_index, account)
        COMMENT 'Antelope account RAM deltas';