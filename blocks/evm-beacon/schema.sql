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
    `signature`                 String COMMENT 'Signature'
)
    ENGINE = ReplacingMergeTree()
    PRIMARY KEY (hash)
    ORDER BY (hash)
    COMMENT 'EVM Beacon block header';
