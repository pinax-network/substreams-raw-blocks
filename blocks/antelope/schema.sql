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
        PRIMARY KEY (hash)
        ORDER BY (hash)
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
        PRIMARY KEY (hash)
        ORDER BY (hash)
        COMMENT 'Antelope transactions';

CREATE TABLE IF NOT EXISTS actions
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'Hash',
    block_date                  Date,

    -- transaction --
    tx_hash                     String COMMENT 'Hash',
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
        PRIMARY KEY (tx_hash, `index`)
        ORDER BY (tx_hash, `index`)
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
    tx_success                  Bool,

    -- action --
    action_index                UInt32,

    -- database operation --
    `index`                     UInt32,
    operation                   LowCardinality(String) COMMENT 'Operation',
    operation_code              UInt8,
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
        PRIMARY KEY (tx_hash, `index`)
        ORDER BY (tx_hash, `index`)
        COMMENT 'Antelope database operations';

CREATE TABLE IF NOT EXISTS feature_ops
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'Hash',
    block_date                  Date,

    -- transaction --
    tx_hash                     String COMMENT 'Hash',
    tx_success                  Bool,

    -- action --
    action_index                UInt32,

    -- feature op --
    feature_digest              String,
    kind                        LowCardinality(String),
    description_digest          String,
    protocol_feature_type       LowCardinality(String),
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (feature_digest)
        ORDER BY (feature_digest)
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
    tx_success                  Bool,

    -- action --
    action_index                UInt32,

    -- perm_op --
    operation                   LowCardinality(String),
    operation_code              UInt8,
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

CREATE TABLE IF NOT EXISTS table_ops
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'Hash',
    block_date                  Date,

    -- transaction --
    tx_hash                     String COMMENT 'Hash',
    tx_success                  Bool,

    -- action --
    action_index                UInt32,

    -- table op --
    `index`                     UInt32,
    operation                   LowCardinality(String),
    operation_code              UInt8,
    payer                       String,
    code                        String,
    scope                       String,
    table_name                  String,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, `index`)
        ORDER BY (tx_hash, `index`)
        COMMENT 'Antelope table operations';

CREATE TABLE IF NOT EXISTS accounts
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'Hash',
    block_date                  Date,

    -- transaction --
    tx_hash                     String COMMENT 'Hash',
    tx_success                  Bool,

    -- action --
    action_index                UInt32,

    -- authority.accounts --
    `index`                     UInt32,
    actor                       String,
    permission                  LowCardinality(String),
    weight                      UInt32,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, action_index, `index`)
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
    tx_success                  Bool,

    -- action --
    action_index                UInt32,

    -- authority.keys --
    `index`                     UInt32,
    public_key                  String,
    weight                      UInt32,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, action_index, `index`)
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
    tx_success                  Bool,

    -- action --
    action_index                UInt32,

    -- authority.waits --
    `index`                     UInt32,
    wait_sec                    UInt32,
    weight                      UInt32,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, action_index, `index`)
        ORDER BY (tx_hash, action_index, `index`)
        COMMENT 'Antelope authority waits';

CREATE TABLE IF NOT EXISTS ram_ops
(
    -- clock --
    block_time    DateTime64(3, 'UTC'),
    block_number  UInt64,
    block_hash    String COMMENT 'Hash',
    block_date    Date,

    -- transaction --
    tx_hash         String COMMENT 'Hash',
    tx_success      Bool,

    -- action --
    action_index    UInt32,

    -- RAM operation --
    operation       LowCardinality(String),
    operation_code  UInt8,
    payer           String,
    delta           Int64,
    usage           UInt64,
    namespace       LowCardinality(String),
    namespace_code  UInt8,
    action          LowCardinality(String),
    action_code     UInt8,
    unique_key      String
)
    ENGINE = ReplacingMergeTree()
    PRIMARY KEY (tx_hash, action_index, unique_key)
    ORDER BY (tx_hash, action_index, unique_key)
    COMMENT 'Antelope RAM operations';

CREATE TABLE IF NOT EXISTS authorizations
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- transaction --
    tx_hash                     String,
    tx_success                  Bool,

    -- action --
    action_index                UInt32,

    -- authorization --
    `index`                     UInt32,
    actor                       String,
    permission                  LowCardinality(String)
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, action_index, `index`)
        ORDER BY (tx_hash, action_index, `index`)
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
    tx_success                  Bool,

    -- action --
    action_index                UInt32,

    -- auth_sequence --
    `index`                     UInt32,
    account_name                String,
    sequence                    UInt64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, action_index, `index`)
        ORDER BY (tx_hash, action_index, `index`)
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
    tx_success                  Bool,

    -- action --
    action_index                UInt32,

    -- account_ram_delta --
    `index`                     UInt32,
    account                     String,
    delta                       Int64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, action_index, `index`)
        ORDER BY (tx_hash, action_index, `index`)
        COMMENT 'Antelope account RAM deltas';

CREATE TABLE IF NOT EXISTS creation_tree
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String,
    block_date                  Date,

    -- transaction --
    tx_hash                     String COMMENT 'Hash',
    tx_success                  Bool,

    -- transaction.creation_tree --
    creator_action_index        Int32,
    execution_action_index      UInt32
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (tx_hash, creator_action_index, execution_action_index)
        ORDER BY (tx_hash, creator_action_index, execution_action_index)
        COMMENT 'Antelope creation tree';

-- Projections --
-- https://clickhouse.com/docs/en/sql-reference/statements/alter/projection --
ALTER TABLE blocks ADD PROJECTION IF NOT EXISTS blocks_by_block_number (
    SELECT * ORDER BY date, number
);
ALTER TABLE transactions ADD PROJECTION IF NOT EXISTS transactions_by_block_number (
    SELECT * ORDER BY block_date, block_number
);
ALTER TABLE actions ADD PROJECTION IF NOT EXISTS actions_by_block_number (
    SELECT * ORDER BY block_date, block_number
);
ALTER TABLE db_ops ADD PROJECTION IF NOT EXISTS db_ops_by_block_number (
    SELECT * ORDER BY block_date, block_number
);
ALTER TABLE feature_ops ADD PROJECTION IF NOT EXISTS feature_ops_by_block_number (
    SELECT * ORDER BY block_date, block_number
);
ALTER TABLE perm_ops ADD PROJECTION IF NOT EXISTS perm_ops_by_block_number (
    SELECT * ORDER BY block_date, block_number
);
ALTER TABLE table_ops ADD PROJECTION IF NOT EXISTS table_ops_by_block_number (
    SELECT * ORDER BY block_date, block_number
);
ALTER TABLE accounts ADD PROJECTION IF NOT EXISTS accounts_by_block_number (
    SELECT * ORDER BY block_date, block_number
);
ALTER TABLE keys ADD PROJECTION IF NOT EXISTS keys_by_block_number (
    SELECT * ORDER BY block_date, block_number
);
ALTER TABLE waits ADD PROJECTION IF NOT EXISTS waits_by_block_number (
    SELECT * ORDER BY block_date, block_number
);
ALTER TABLE ram_ops ADD PROJECTION IF NOT EXISTS ram_ops_by_block_number (
    SELECT * ORDER BY block_date, block_number
);
ALTER TABLE authorizations ADD PROJECTION IF NOT EXISTS authorizations_by_block_number (
    SELECT * ORDER BY block_date, block_number
);
ALTER TABLE auth_sequences ADD PROJECTION IF NOT EXISTS auth_sequences_by_block_number (
    SELECT * ORDER BY block_date, block_number
);
ALTER TABLE account_ram_deltas ADD PROJECTION IF NOT EXISTS account_ram_deltas_by_block_number (
    SELECT * ORDER BY block_date, block_number
);
ALTER TABLE creation_tree ADD PROJECTION IF NOT EXISTS creation_tree_by_block_number (
    SELECT * ORDER BY block_date, block_number
);

ALTER TABLE blocks MATERIALIZE PROJECTION blocks_by_block_number;
ALTER TABLE transactions MATERIALIZE PROJECTION transactions_by_block_number;
ALTER TABLE feature_ops MATERIALIZE PROJECTION feature_ops_by_block_number;
ALTER TABLE perm_ops MATERIALIZE PROJECTION perm_ops_by_block_number;
ALTER TABLE table_ops MATERIALIZE PROJECTION table_ops_by_block_number;
ALTER TABLE accounts MATERIALIZE PROJECTION accounts_by_block_number;
ALTER TABLE keys MATERIALIZE PROJECTION keys_by_block_number;
ALTER TABLE waits MATERIALIZE PROJECTION waits_by_block_number;
ALTER TABLE ram_ops MATERIALIZE PROJECTION ram_ops_by_block_number;
ALTER TABLE actions MATERIALIZE PROJECTION actions_by_block_number;
ALTER TABLE db_ops MATERIALIZE PROJECTION db_ops_by_block_number;
ALTER TABLE authorizations MATERIALIZE PROJECTION authorizations_by_block_number;
ALTER TABLE auth_sequences MATERIALIZE PROJECTION auth_sequences_by_block_number;
ALTER TABLE account_ram_deltas MATERIALIZE PROJECTION account_ram_deltas_by_block_number;
ALTER TABLE creation_tree MATERIALIZE PROJECTION creation_tree_by_block_number;