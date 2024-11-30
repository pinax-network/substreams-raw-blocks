# `Antelope`: Snowflake Datashare

## Data Dictionary

### `blocks`

<details>
<summary>Click to expand Snowflake Table Schema for "blocks"</summary>

| Column Name                         | Data Type           | Description                                                                   |
| ----------------------------------- | ------------------- | ----------------------------------------------------------------------------- |
| time                                | TIMESTAMP_NTZ(3)    | Timestamp indicating when this block was created                              |
| number                              | NUMBER(38,0)        | Sequential number of the block in the blockchain                              |
| date                                | DATE                | Calendar date associated with the block                                       |
| hash                                | VARCHAR             | Unique identifier (hash) of the block                                         |
| parent_hash                         | VARCHAR             | Hash of the parent block, linking this block to its predecessor               |
| producer                            | VARCHAR             | Name of the producer responsible for creating this block                      |
| confirmed                           | NUMBER(10,0)        | Number of confirmations for this block                                        |
| schedule_version                    | NUMBER(10,0)        | Version of the block producer schedule in effect                              |
| version                             | NUMBER(10,0)        | Version of the block as defined by the protocol                               |
| producer_signature                  | VARCHAR             | Signature of the producer who created this block                              |
| dpos_proposed_irreversible_blocknum | NUMBER(10,0)        | Last block number proposed as irreversible by delegated proof of stake (DPoS) |
| dpos_irreversible_blocknum          | NUMBER(10,0)        | Last block number considered irreversible by DPoS consensus                   |
| transaction_mroot                   | VARCHAR             | Merkle root of all transactions in this block                                 |
| action_mroot                        | VARCHAR             | Merkle root of all actions in this block                                      |
| blockroot_merkle_active_nodes       | ARRAY<VARCHAR>      | List of active nodes in the blockroot Merkle tree                             |
| blockroot_merkle_node_count         | NUMBER(10,0)        | Number of nodes in the blockroot Merkle tree                                  |
| action_mroot_savanna                | VARCHAR             | Secondary Merkle root for actions, specific to the "Savanna" protocol         |
| block_signing_key                   | VARCHAR             | Block signing key used by the producer                                        |
| confirm_count                       | ARRAY<NUMBER(10,0)> | Count of confirmations received from other producers for this block           |
| size                                | NUMBER(38,0)        | Size of the block in bytes                                                    |
| total_transactions                  | NUMBER(38,0)        | Total number of transactions included in this block                           |
| successful_transactions             | NUMBER(38,0)        | Number of successfully processed transactions in this block                   |
| failed_transactions                 | NUMBER(38,0)        | Number of failed transactions in this block                                   |
| total_actions                       | NUMBER(38,0)        | Total number of actions included in this block                                |
| total_db_ops                        | NUMBER(38,0)        | Total number of database operations executed in this block                    |

</details>


### `transactions`

<details>
<summary>Click to expand Snowflake Table Schema for "transactions"</summary>


| Column Name              | Data Type           | Description                                                                  |
| ------------------------ | ------------------- | ---------------------------------------------------------------------------- |
| block_time               | TIMESTAMP_NTZ(3)    | Timestamp indicating when the block containing this transaction was created  |
| block_number             | NUMBER(38,0)        | Sequential number of the block in the blockchain containing this transaction |
| block_hash               | VARCHAR             | Unique identifier (hash) of the block containing this transaction            |
| block_date               | DATE                | Calendar date associated with the block containing this transaction          |
| hash                     | VARCHAR             | Unique identifier (hash) of the transaction                                  |
| index                    | NUMBER(38,0)        | Index of the transaction within the block                                    |
| elapsed                  | NUMBER(38,0)        | Time elapsed during the execution of the transaction in microseconds         |
| net_usage                | NUMBER(38,0)        | Net bandwidth usage of the transaction in bytes                              |
| scheduled                | BOOLEAN             | Indicates whether the transaction was scheduled or not                       |
| cpu_usage_micro_seconds  | NUMBER(10,0)        | CPU time used by the transaction in microseconds                             |
| net_usage_words          | NUMBER(10,0)        | Net bandwidth usage of the transaction in 64-bit words                       |
| status                   | VARCHAR             | Status of the transaction as a string                                        |
| status_code              | NUMBER(10,0)        | Numeric code representing the status of the transaction                      |
| success                  | BOOLEAN             | Indicates whether the transaction was successfully executed                  |
| transaction_mroot        | VARCHAR             | Merkle root of the transaction                                               |
| creator_action_indexes   | ARRAY<NUMBER(10,0)> | List of indexes for actions created by this transaction                      |
| execution_action_indexes | ARRAY<NUMBER(10,0)> | List of indexes for actions executed by this transaction                     |

</details>

### `actions`

<details>
<summary>Click to expand Snowflake Table Schema for "actions"</summary>

| Column Name                                | Data Type           | Description                                                                   |
| ------------------------------------------ | ------------------- | ----------------------------------------------------------------------------- |
| block_time                                 | TIMESTAMP_NTZ(3)    | Timestamp indicating when the block containing this action was created        |
| block_number                               | NUMBER(38,0)        | Sequential number of the block in the blockchain containing this action       |
| block_hash                                 | VARCHAR             | Unique identifier (hash) of the block containing this action                  |
| block_date                                 | DATE                | Calendar date associated with the block containing this action                |
| tx_hash                                    | VARCHAR             | Unique identifier (hash) of the transaction containing this action            |
| tx_success                                 | BOOLEAN             | Indicates whether the transaction containing this action was successful       |
| abi_sequence                               | NUMBER(38,0)        | Sequence number of the ABI (Application Binary Interface)                     |
| code_sequence                              | NUMBER(38,0)        | Sequence number of the code associated with this action                       |
| digest                                     | VARCHAR             | Digest hash of the action receipt                                             |
| global_sequence                            | NUMBER(38,0)        | Global sequence number for the action across the blockchain                   |
| receipt_receiver                           | VARCHAR             | Account name that received the action                                         |
| recv_sequence                              | NUMBER(38,0)        | Sequence number of the received action for the receiver                       |
| auth_sequence                              | ARRAY<NUMBER(38,0)> | Sequence numbers for each authorizer of the action                            |
| auth_sequence_account_name                 | ARRAY<VARCHAR>      | Account names of the authorizers corresponding to `auth_sequence`             |
| account                                    | VARCHAR             | Name of the account that performed the action                                 |
| name                                       | VARCHAR             | Name of the action (eg, "transfer")                                           |
| json_data                                  | VARCHAR             | JSON representation of the action's data                                      |
| raw_data                                   | VARCHAR             | Raw hex representation of the action's data                                   |
| index                                      | NUMBER(10,0)        | Index of the action within the transaction                                    |
| action_ordinal                             | NUMBER(10,0)        | Ordinal position of the action within the transaction                         |
| receiver                                   | VARCHAR             | Account name that received the action                                         |
| context_free                               | BOOLEAN             | Indicates whether the action is context-free (does not require authorization) |
| elapsed                                    | NUMBER(38,0)        | Time elapsed during the execution of the action in microseconds               |
| console                                    | VARCHAR             | Console output produced by the action during execution                        |
| raw_return_value                           | VARCHAR             | Raw representation of the action's return value                               |
| json_return_value                          | VARCHAR             | JSON representation of the action's return value                              |
| creator_action_ordinal                     | NUMBER(10,0)        | Ordinal position of the action that created this action, if applicable        |
| closest_unnotified_ancestor_action_ordinal | NUMBER(10,0)        | Ordinal position of the closest ancestor action not notified by this action   |
| action_mroot                               | VARCHAR             | Merkle root of the actions in this block                                      |
| account_ram_deltas_account                 | ARRAY<VARCHAR>      | Account names associated with RAM deltas for this action                      |
| account_ram_deltas                         | ARRAY<NUMBER(38,0)> | Amounts of RAM (in bytes) added or removed for each account                   |

</details>

### `db_ops`

<details>
<summary>Click to expand Snowflake Table Schema for "db_ops"</summary>

| Column Name    | Data Type        | Description                                                                        |
| -------------- | ---------------- | ---------------------------------------------------------------------------------- |
| block_time     | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this database operation was created |
| block_number   | NUMBER(38,0)     | Sequential number of the block in the blockchain containing this operation         |
| block_hash     | VARCHAR          | Unique identifier (hash) of the block containing this operation                    |
| block_date     | DATE             | Calendar date associated with the block containing this operation                  |
| tx_hash        | VARCHAR          | Unique identifier (hash) of the transaction containing this operation              |
| tx_success     | BOOLEAN          | Indicates whether the transaction containing this operation was successful         |
| action_index   | NUMBER(10,0)     | Index of the action within the transaction associated with this operation          |
| index          | NUMBER(10,0)     | Index of this database operation within the action                                 |
| operation      | VARCHAR          | Type of database operation (eg, "insert", "update", "delete")                      |
| operation_code | NUMBER(10,0)     | Numeric code representing the type of database operation                           |
| code           | VARCHAR          | Code or contract namespace associated with this database operation                 |
| scope          | VARCHAR          | Scope of the database operation, specifying the namespace or partition affected    |
| table_name     | VARCHAR          | Name of the database table affected by this operation                              |
| primary_key    | VARCHAR          | Primary key of the record affected by this operation                               |
| old_payer      | VARCHAR          | Name of the payer responsible for RAM costs before the operation                   |
| new_payer      | VARCHAR          | Name of the payer responsible for RAM costs after the operation                    |
| old_data       | VARCHAR          | Raw data before the operation                                                      |
| new_data       | VARCHAR          | Raw data after the operation                                                       |
| old_data_json  | VARCHAR          | JSON representation of the data before the operation                               |
| new_data_json  | VARCHAR          | JSON representation of the data after the operation                                |

</details>

### `feature_ops`

<details>
<summary>Click to expand Snowflake Table Schema for "feature_ops"</summary>

| Column Name           | Data Type        | Description                                                                       |
| --------------------- | ---------------- | --------------------------------------------------------------------------------- |
| block_time            | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this feature operation was created |
| block_number          | NUMBER(38,0)     | Sequential number of the block in the blockchain containing this operation        |
| block_hash            | VARCHAR          | Unique identifier (hash) of the block containing this operation                   |
| block_date            | DATE             | Calendar date associated with the block containing this operation                 |
| tx_hash               | VARCHAR          | Unique identifier (hash) of the transaction containing this operation             |
| tx_success            | BOOLEAN          | Indicates whether the transaction containing this operation was successful        |
| action_index          | NUMBER(10,0)     | Index of the action within the transaction associated with this operation         |
| feature_digest        | VARCHAR          | Digest hash representing the feature being operated on                            |
| kind                  | VARCHAR          | Type or kind of feature operation (eg, "activate", "deactivate")                  |
| description_digest    | VARCHAR          | Digest hash for the description of the feature being operated on                  |
| protocol_feature_type | VARCHAR          | Type of protocol feature associated with this operation (eg, "consensus")         |

</details>

### `perm_ops`

<details>
<summary>Click to expand Snowflake Table Schema for "perm_ops"</summary>

| Column Name     | Data Type           | Description                                                                          |
| --------------- | ------------------- | ------------------------------------------------------------------------------------ |
| block_time      | TIMESTAMP_NTZ(3)    | Timestamp indicating when the block containing this permission operation was created |
| block_number    | NUMBER(38,0)        | Sequential number of the block in the blockchain containing this operation           |
| block_hash      | VARCHAR             | Unique identifier (hash) of the block containing this operation                      |
| block_date      | DATE                | Calendar date associated with the block containing this operation                    |
| tx_hash         | VARCHAR             | Unique identifier (hash) of the transaction containing this operation                |
| tx_success      | BOOLEAN             | Indicates whether the transaction containing this operation was successful           |
| action_index    | NUMBER(10,0)        | Index of the action within the transaction associated with this operation            |
| operation       | VARCHAR             | Type of permission operation (eg, "create", "update", "delete")                      |
| operation_code  | NUMBER(10,0)        | Numeric code representing the type of permission operation                           |
| id              | NUMBER(38,0)        | Unique identifier for the permission record being operated on                        |
| parent_id       | NUMBER(38,0)        | Identifier of the parent permission, if applicable                                   |
| owner           | VARCHAR             | Account name that owns the permission                                                |
| name            | VARCHAR             | Name of the permission (eg, "active", "owner")                                       |
| threshold       | NUMBER(10,0)        | Threshold required for actions under this permission to execute                      |
| accounts        | ARRAY<VARCHAR>      | List of account names associated with this permission                                |
| accounts_weight | ARRAY<NUMBER(10,0)> | List of weights assigned to the accounts under this permission                       |
| keys_public_key | ARRAY<VARCHAR>      | List of public keys associated with this permission                                  |
| keys_weight     | ARRAY<NUMBER(10,0)> | List of weights assigned to the keys under this permission                           |
| wait_sec        | ARRAY<NUMBER(10,0)> | List of wait times in seconds for this permission                                    |
| wait_weight     | ARRAY<NUMBER(10,0)> | List of weights assigned to the wait times under this permission                     |

</details>

### `table_ops`

<details>
<summary>Click to expand Snowflake Table Schema for "table_ops"</summary>

| Column Name    | Data Type        | Description                                                                     |
| -------------- | ---------------- | ------------------------------------------------------------------------------- |
| block_time     | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this table operation was created |
| block_number   | NUMBER(38,0)     | Sequential number of the block in the blockchain containing this operation      |
| block_hash     | VARCHAR          | Unique identifier (hash) of the block containing this operation                 |
| block_date     | DATE             | Calendar date associated with the block containing this operation               |
| tx_hash        | VARCHAR          | Unique identifier (hash) of the transaction containing this operation           |
| tx_success     | BOOLEAN          | Indicates whether the transaction containing this operation was successful      |
| action_index   | NUMBER(10,0)     | Index of the action within the transaction associated with this operation       |
| index          | NUMBER(10,0)     | Index of the table operation within the action                                  |
| operation      | VARCHAR          | Type of table operation (eg, "create", "update", "delete")                      |
| operation_code | NUMBER(10,0)     | Numeric code representing the type of table operation                           |
| payer          | VARCHAR          | Name of the payer responsible for RAM costs associated with the operation       |
| code           | VARCHAR          | Code or contract namespace associated with the table operation                  |
| scope          | VARCHAR          | Scope of the table operation, specifying the namespace or partition affected    |
| table_name     | VARCHAR          | Name of the table affected by this operation                                    |

</details>

### `ram_ops`

<details>
<summary>Click to expand Snowflake Table Schema for "ram_ops"</summary>

| Column Name    | Data Type        | Description                                                                   |
| -------------- | ---------------- | ----------------------------------------------------------------------------- |
| block_time     | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this RAM operation was created |
| block_number   | NUMBER(38,0)     | Sequential number of the block in the blockchain containing this operation    |
| block_hash     | VARCHAR          | Unique identifier (hash) of the block containing this operation               |
| block_date     | DATE             | Calendar date associated with the block containing this operation             |
| tx_hash        | VARCHAR          | Unique identifier (hash) of the transaction containing this operation         |
| tx_success     | BOOLEAN          | Indicates whether the transaction containing this operation was successful    |
| action_index   | NUMBER(10,0)     | Index of the action within the transaction associated with this operation     |
| operation      | VARCHAR          | Type of RAM operation (eg, "allocate", "free")                                |
| operation_code | NUMBER(10,0)     | Numeric code representing the type of RAM operation                           |
| payer          | VARCHAR          | Name of the payer responsible for the RAM costs in this operation             |
| delta          | NUMBER(38,0)     | Change in RAM usage in bytes resulting from this operation                    |
| usage          | NUMBER(38,0)     | Total RAM usage in bytes after this operation                                 |
| namespace      | VARCHAR          | Namespace associated with this RAM operation (eg, account or contract)        |
| namespace_code | NUMBER(10,0)     | Numeric code representing the namespace of the operation                      |
| action         | VARCHAR          | Name of the action associated with this operation                             |
| action_code    | NUMBER(10,0)     | Numeric code representing the action type                                     |
| unique_key     | VARCHAR          | Unique identifier or key associated with this RAM operation                   |

</details>