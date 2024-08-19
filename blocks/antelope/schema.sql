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
    previous                                String COMMENT 'Hash',
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
    total_transactions                      UInt64,
    successful_transactions                 UInt64,
    failed_transactions                     UInt64,
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (date, number)
        ORDER BY (date, number, hash)
        COMMENT 'Antelope block header';

CREATE TABLE IF NOT EXISTS transactions
(
    -- block --
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

CREATE TABLE IF NOT EXISTS traces
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
	receiver                                        String,
	context_free                                    Bool,
	elapsed                                         Int64,
	console                                         String,
	raw_return_value                                String,
	json_return_value                               String,
	action_ordinal                                  UInt32,
	creator_action_ordinal                          UInt32,
	closest_unnotified_ancestor_action_ordinal      UInt32,
	execution_index                                 UInt32,

    -- block roots --
    action_mroot                                    String COMMENT 'Hash',
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, action_ordinal)
        COMMENT 'Antelope traces';

CREATE TABLE IF NOT EXISTS storage_changes
(
    -- block --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_hash                  String COMMENT 'EVM Hash',
    block_date                  Date,

    -- storage change --
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
    ENGINE = MergeTree()
        PRIMARY KEY (block_date, block_number)
        ORDER BY (block_date, block_number, block_hash, action_index)
        COMMENT 'Antelope storage changes';
