# `Cosmos`: Snowflake Datashare

## Data Dictionary

### `blocks`

<details>
<summary>Click to expand Snowflake Table Schema for "blocks"</summary>

| Column Name             | Data Type        | Description                                               |
| ----------------------- | ---------------- | --------------------------------------------------------- |
| time                    | TIMESTAMP_NTZ(3) | Timestamp indicating when the block was created           |
| number                  | NUMBER(38,0)     | Height of the block in the Cosmos chain                   |
| date                    | DATE             | Calendar date associated with the block                   |
| hash                    | VARCHAR          | Unique identifier (hash) of the block                     |
| version_consensus_block | NUMBER(38,0)     | Version of the consensus protocol for blocks              |
| version_consensus_app   | NUMBER(38,0)     | Version of the consensus protocol for applications        |
| chain_id                | VARCHAR          | Identifier of the blockchain chain                        |
| last_block_id           | VARCHAR          | Identifier of the last block in the chain                 |
| last_commit_hash        | VARCHAR          | Hash of the last block's commit                           |
| data_hash               | VARCHAR          | Hash of the block's data (transactions)                   |
| validators_hash         | VARCHAR          | Hash of the current validator set                         |
| next_validators_hash    | VARCHAR          | Hash of the next validator set                            |
| consensus_hash          | VARCHAR          | Hash of the consensus parameters for this block           |
| app_hash                | VARCHAR          | Hash of the application state after applying this block   |
| last_results_hash       | VARCHAR          | Hash of the results of the last block's transactions      |
| evidence_hash           | VARCHAR          | Hash of any evidence of misbehavior in this block         |
| proposer_address        | VARCHAR          | Address of the validator proposing this block             |
| total_transactions      | NUMBER(38,0)     | Total number of transactions in the block                 |
| successful_transactions | NUMBER(38,0)     | Number of successfully executed transactions in the block |
| failed_transactions     | NUMBER(38,0)     | Number of transactions that failed in the block           |

</details>

### `transactions`

<details>
<summary>Click to expand Snowflake Table Schema for "transactions"</summary>

| Column Name  | Data Type        | Description                                                                 |
| ------------ | ---------------- | --------------------------------------------------------------------------- |
| block_time   | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this transaction was created |
| block_number | NUMBER(38,0)     | Height of the block in the Cosmos chain containing this transaction         |
| block_date   | DATE             | Calendar date associated with the block containing this transaction         |
| block_hash   | VARCHAR          | Unique identifier (hash) of the block containing this transaction           |
| index        | NUMBER(10,0)     | Index of the transaction within the block                                   |
| hash         | VARCHAR          | Unique identifier (hash) of the transaction                                 |
| code         | NUMBER(10,0)     | Code representing the transaction's execution status                        |
| data         | VARCHAR          | Data associated with the transaction                                        |
| log          | VARCHAR          | Log of events generated during the transaction's execution                  |
| info         | VARCHAR          | Additional informational messages about the transaction's execution         |
| gas_wanted   | NUMBER(38,0)     | Amount of gas requested for the transaction's execution                     |
| gas_used     | NUMBER(38,0)     | Actual amount of gas used for the transaction's execution                   |
| codespace    | VARCHAR          | Namespace for categorizing errors related to the transaction                |

</details>

### `transaction_events`

<details>
<summary>Click to expand Snowflake Table Schema for "transaction_events"</summary>

| Column Name  | Data Type        | Description                                                                       |
| ------------ | ---------------- | --------------------------------------------------------------------------------- |
| block_time   | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this transaction event was created |
| block_number | NUMBER(38,0)     | Height of the block in the Cosmos chain containing this transaction event         |
| block_date   | DATE             | Calendar date associated with the block containing this transaction event         |
| block_hash   | VARCHAR          | Unique identifier (hash) of the block containing this transaction event           |
| tx_hash      | VARCHAR          | Unique identifier (hash) of the transaction associated with this event            |
| index        | NUMBER(10,0)     | Index of the event within the transaction                                         |
| type         | VARCHAR          | Type of the event                                                                 |
| attributes   | ARRAY<VARCHAR>   | List of attributes associated with the event, represented as key-value pairs      |

</details>

### `block_events`

<details>
<summary>Click to expand Snowflake Table Schema for "block_events"</summary>

| Column Name  | Data Type        | Description                                                                  |
| ------------ | ---------------- | ---------------------------------------------------------------------------- |
| block_time   | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this event was created        |
| block_number | NUMBER(38,0)     | Height of the block in the Cosmos chain containing this event                |
| block_date   | DATE             | Calendar date associated with the block containing this event                |
| block_hash   | VARCHAR          | Unique identifier (hash) of the block containing this event                  |
| index        | NUMBER(10,0)     | Index of the event within the block                                          |
| type         | VARCHAR          | Type of the event (eg, "block_start", "block_end")                           |
| attributes   | ARRAY<VARCHAR>   | List of attributes associated with the event, represented as key-value pairs |

</details>

### `misbehaviors`

<details>
<summary>Click to expand Snowflake Table Schema for "misbehaviors"</summary>

| Column Name        | Data Type        | Description                                                                 |
| ------------------ | ---------------- | --------------------------------------------------------------------------- |
| block_time         | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this misbehavior was created |
| block_number       | NUMBER(38,0)     | Height of the block in the Cosmos chain containing this misbehavior         |
| block_date         | DATE             | Calendar date associated with the block containing this misbehavior         |
| block_hash         | VARCHAR          | Unique identifier (hash) of the block containing this misbehavior           |
| index              | NUMBER(10,0)     | Index of the misbehavior entry within the block                             |
| type               | VARCHAR          | Type of misbehavior (eg, "double_signing")                                  |
| validator_address  | VARCHAR          | Address of the validator committing the misbehavior                         |
| validator_power    | NUMBER(38,0)     | Voting power of the validator at the time of misbehavior                    |
| height             | NUMBER(38,0)     | Block height where the misbehavior occurred                                 |
| time               | TIMESTAMP_NTZ(3) | Time when the misbehavior occurred                                          |
| total_voting_power | NUMBER(38,0)     | Total voting power of all validators at the time of misbehavior             |

</details>

### `consensus_param_updates`

<details>
<summary>Click to expand Snowflake Table Schema for "consensus_param_updates"</summary>

| Column Name                 | Data Type        | Description                                                                      |
| --------------------------- | ---------------- | -------------------------------------------------------------------------------- |
| block_time                  | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this consensus update was created |
| block_number                | NUMBER(38,0)     | Height of the block in the Cosmos chain containing this consensus update         |
| block_date                  | DATE             | Calendar date associated with the block containing this consensus update         |
| block_hash                  | VARCHAR          | Unique identifier (hash) of the block containing this consensus update           |
| block_max_bytes             | NUMBER(38,0)     | Maximum number of bytes allowed in a block                                       |
| block_max_gas               | NUMBER(38,0)     | Maximum gas allowed in a block                                                   |
| evidence_max_age_num_blocks | NUMBER(38,0)     | Maximum age of evidence in terms of block numbers                                |
| evidence_max_age_duration   | VARCHAR          | Maximum age of evidence in terms of time duration                                |
| evidence_max_bytes          | NUMBER(38,0)     | Maximum bytes of evidence allowed in the block                                   |
| validator_pub_key_types     | ARRAY<VARCHAR>   | List of allowed public key types for validators                                  |
| app_version                 | NUMBER(38,0)     | Version of the application associated with the consensus parameters              |

</details>

### `transaction_messages`

<details>
<summary>Click to expand Snowflake Table Schema for "transaction_messages"</summary>

| Column Name  | Data Type        | Description                                                                         |
| ------------ | ---------------- | ----------------------------------------------------------------------------------- |
| block_time   | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this transaction message was created |
| block_number | NUMBER(38,0)     | Height of the block in the Cosmos chain containing this transaction message         |
| block_date   | DATE             | Calendar date associated with the block containing this transaction message         |
| block_hash   | VARCHAR          | Unique identifier (hash) of the block containing this transaction message           |
| tx_hash      | VARCHAR          | Unique identifier (hash) of the transaction associated with this message            |
| index        | NUMBER(10,0)     | Index of the message within the transaction                                         |
| type         | VARCHAR          | Type of the message                                                                 |
| value        | VARCHAR          | Raw hexadecimal representation of the message value                                 |

</details>







