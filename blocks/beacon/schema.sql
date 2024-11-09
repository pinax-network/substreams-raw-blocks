CREATE TABLE IF NOT EXISTS blocks
(
    -- clock --
    time                        DateTime64(3, 'UTC'),
    number                      UInt64,
    date                        Date,
    hash                        String COMMENT 'EVM Hash',

    -- block --
    `version`                   UInt64,
    spec                        LowCardinality(String) COMMENT 'Specification version',
    slot                        UInt64,
    parent_slot                 UInt64,
    `root`                      String COMMENT 'Root hash',
    parent_root                 String COMMENT 'Parent hash',
    state_root                  String COMMENT 'State root hash',
    proposer_index              UInt64,
    body_root                   String COMMENT 'Body root hash',
    `signature`                 String COMMENT 'Signature',
)
    ENGINE = ReplacingMergeTree()
    PRIMARY KEY (hash)
    ORDER BY (hash)
    COMMENT 'EVM Beacon block header';

CREATE TABLE IF NOT EXISTS blobs
(
    -- clock --
    block_time                          DateTime64(3, 'UTC'),
    block_number                        UInt64,
    block_date                          Date,
    block_hash                          String COMMENT 'EVM Hash',

    -- blob --
    `index`                             UInt64,
    blob                                String,
    kzg_commitment                      String,
    kzg_proof                           String,
    kzg_commitment_inclusion_proof      Array(String),
)
    ENGINE = ReplacingMergeTree()
    PRIMARY KEY (block_hash, index)
    ORDER BY (block_hash, index)
    COMMENT 'EVM Beacon block blobs';

CREATE TABLE IF NOT EXISTS deposits
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_date                  Date,
    block_hash                  String COMMENT 'EVM Hash',

    -- deposit --
    `index`                     UInt64 COMMENT 'Deposit index within block',
    proof                       Array(String),
    pubkey                      String,
    withdrawal_credentials      String,
    signature                   String,
    gwei                        UInt64
)
    ENGINE = ReplacingMergeTree()
    PRIMARY KEY (block_hash, index)
    ORDER BY (block_hash, index)
    COMMENT 'EVM Beacon block deposits';

CREATE TABLE IF NOT EXISTS withdrawals
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_date                  Date,
    block_hash                  String COMMENT 'EVM Hash',

    -- withdrawal --
    withdrawal_index            UInt64,
    validator_index             UInt64,
    `address`                    String,
    gwei                        UInt64
)
    ENGINE = ReplacingMergeTree()
    PRIMARY KEY (block_hash, withdrawal_index)
    ORDER BY (block_hash, withdrawal_index)
    COMMENT 'EVM Beacon block withdrawals';

CREATE TABLE IF NOT EXISTS attestations
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_date                  Date,
    block_hash                  String COMMENT 'EVM Hash',

    -- attestation --
    `index`                     UInt64 COMMENT 'Attestation index within block',
    aggregation_bits            String,
    slot                        UInt64,
    committee_index             UInt64,
    beacon_block_root           String,
    source_epoch                UInt64,
    source_root                 String,
    target_epoch                UInt64,
    target_root                 String,
    signature                   String
)
    ENGINE = ReplacingMergeTree()
    PRIMARY KEY (block_hash, index)
    ORDER BY (block_hash, index)
    COMMENT 'EVM Beacon block attestations';

CREATE TABLE IF NOT EXISTS attester_slashings
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_date                  Date,
    block_hash                  String COMMENT 'EVM Hash',

    -- attester slashing --
    `index`                     UInt64 COMMENT 'Attester slashing index within block',

    -- attestation 1 --
    attestation_1_attesting_indices String,
    attestation_1_slot              UInt64,
    attestation_1_committee_index   UInt64,
    attestation_1_beacon_block_root String,
    attestation_1_source_epoch      UInt64,
    attestation_1_source_root       String,
    attestation_1_target_epoch      UInt64,
    attestation_1_target_root       String,
    attestation_1_signature         String,

    -- attestation 2 --
    attestation_2_attesting_indices String,
    attestation_2_slot             UInt64,
    attestation_2_committee_index   UInt64,
    attestation_2_beacon_block_root String,
    attestation_2_source_epoch      UInt64,
    attestation_2_source_root       String,
    attestation_2_target_epoch      UInt64,
    attestation_2_target_root       String,
    attestation_2_signature         String
)
    ENGINE = ReplacingMergeTree()
    PRIMARY KEY (block_hash)
    ORDER BY (block_hash)
    COMMENT 'EVM Beacon block attester slashings';

CREATE TABLE IF NOT EXISTS bls_to_execution_changes
(
    -- clock --
    block_time                  DateTime64(3, 'UTC'),
    block_number                UInt64,
    block_date                  Date,
    block_hash                  String COMMENT 'EVM Hash',

    -- bls to execution change --
    `index`                     UInt64 COMMENT 'BLS to execution change index within block',
    validator_index             UInt64,
    from_bls_pubkey             String,
    to_execution_address        String,
    signature                   String
)
    ENGINE = ReplacingMergeTree()
    PRIMARY KEY (block_hash, index)
    ORDER BY (block_hash, index)
    COMMENT 'EVM Beacon block BLS to execution changes';

-- Projections --

ALTER TABLE blocks ADD PROJECTION IF NOT EXISTS blocks_by_block_height (
    SELECT * ORDER BY date, number
);

ALTER TABLE blobs ADD PROJECTION IF NOT EXISTS blobs_by_block_height (
    SELECT * ORDER BY block_date, block_number
);

ALTER TABLE deposits ADD PROJECTION IF NOT EXISTS deposits_by_block_height (
    SELECT * ORDER BY block_date, block_number
);

ALTER TABLE withdrawals ADD PROJECTION IF NOT EXISTS withdrawals_by_block_height (
    SELECT * ORDER BY block_date, block_number
);

ALTER TABLE attestations ADD PROJECTION IF NOT EXISTS attestations_by_block_height (
    SELECT * ORDER BY block_date, block_number
);

ALTER TABLE attester_slashings ADD PROJECTION IF NOT EXISTS attester_slashings_by_block_height (
    SELECT * ORDER BY block_date, block_number
);

ALTER TABLE bls_to_execution_changes ADD PROJECTION IF NOT EXISTS bls_to_execution_changes_by_block_height (
    SELECT * ORDER BY block_date, block_number
);

ALTER TABLE blocks MATERIALIZE PROJECTION blocks_by_block_height;

ALTER TABLE blobs MATERIALIZE PROJECTION blobs_by_block_height;

ALTER TABLE deposits MATERIALIZE PROJECTION deposits_by_block_height;

ALTER TABLE withdrawals MATERIALIZE PROJECTION withdrawals_by_block_height;

ALTER TABLE attestations MATERIALIZE PROJECTION attestations_by_block_height;

ALTER TABLE attester_slashings MATERIALIZE PROJECTION attester_slashings_by_block_height;

ALTER TABLE bls_to_execution_changes MATERIALIZE PROJECTION bls_to_execution_changes_by_block_height;
