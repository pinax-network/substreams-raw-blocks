# `Antelope`: Snowflake Datashare

> Chains supported `EOS`, `WAX` & `Telos`

This dataset offers a detailed view of Antleope based blockchain activity, providing critical data points across blocks, transactions, traces, and logs for comprehensive blockchain analysis. Designed for developers, analysts, and researchers, this dataset supports advanced analytics, indexing, and querying capabilities tailored to Antelope's unique structure and behavior.

Free access to the dataset on the [Snowflake Data Marketplace](https://app.snowflake.com/marketplace).

## Tables/Views

| Data Type     | Description |
|---------------|-------------|
| `blocks`      | Block-level data, including time, number, hash, producer, and various roots. |
| `transactions`| Transaction details such as hash, index, status, success indicators, and execution metrics. |
| `actions`     | Action details within transactions, including account information, execution data, and RAM deltas. |
| `db_ops`      | Database operations performed in actions, including operations on tables, scopes, and data changes. |
| `feature_ops` | Feature operations related to protocol features, including feature digests and descriptions. |
| `perm_ops`    | Permission operations, managing authorities, permissions, and related metadata. |
| `table_ops`   | Table operations, including operations on tables, payer information, and scope details. |
| `ram_ops`     | RAM operations, tracking memory usage, operations, and related namespaces. |

## Sample SQL Queries

**Daily Active Users (DAU)**

```sql
SELECT
    block_date,
    count(distinct auth_sequence_account_name) AS user
FROM wax.actions
WHERE index = 0
GROUP BY block_date
ORDER BY block_date ASC
```

**Top Contracts**

```sql
SELECT
    account AS contract,
    count(*) AS actions
FROM wax.actions
WHERE block_date = '2025-01-01'
GROUP BY contract
ORDER BY actions DESC
LIMIT 10
```

**Token Transfers**

```sql
SELECT block_number, block_time, tx_hash,
    GET(json_data, 'from')::string as "from",
    GET(json_data, 'to')::string as "to",
    GET(json_data, 'quantity')::string as quantity,
    GET(json_data, 'memo')::string as memo,
    SPLIT(quantity, ' ')[0]::double as value,
    SPLIT(quantity, ' ')[1]::string as symbol
FROM wax.actions
WHERE account='eosio.token' AND name='transfer' AND account=receiver
ORDER BY block_number DESC
LIMIT 10;
```


**View the first and last block indexed**
> This query tells you how fresh the data is.

```sql
SELECT
  MIN(number) AS "First block",
  MAX(number) AS "Newest block",
  COUNT(1) AS "Total number of blocks"
FROM
  wax.blocks;
```

## Data Dictionary

### `blocks`

<details>
<summary>Click to expand Snowflake Table Schema for "blocks"</summary>

| Field Name                                    | Type             | Description                                               |
|-----------------------------------------------|------------------|-----------------------------------------------------------|
| **time**                                      | TIMESTAMP_NTZ(3) | Timestamp of the block                                    |
| **number**                                    | NUMBER(38,0)     | Block number                                              |
| **date**                                      | DATE             | Date of the block                                         |
| **hash**                                      | VARCHAR          | Block hash                                                |
| **parent_hash**                               | VARCHAR          | Parent block hash                                         |
| **producer**                                  | VARCHAR          | Producer of the block                                     |
| **confirmed**                                 | NUMBER(10,0)     | Confirmation status                                       |
| **schedule_version**                          | NUMBER(10,0)     | Schedule version                                          |
| **version**                                   | NUMBER(10,0)     | Block version                                             |
| **producer_signature**                        | VARCHAR          | Signature of the producer                                 |
| **dpos_proposed_irreversible_blocknum**        | NUMBER(10,0)     | Proposed irreversible block number                        |
| **dpos_irreversible_blocknum**                 | NUMBER(10,0)     | Irreversible block number                                 |
| **transaction_mroot**                         | VARCHAR          | Merkle root of transactions                               |
| **action_mroot**                              | VARCHAR          | Merkle root of actions                                    |
| **blockroot_merkle_active_nodes**              | ARRAY<VARCHAR>   | Active nodes in the blockroot Merkle tree                 |
| **blockroot_merkle_node_count**                | NUMBER(10,0)     | Number of nodes in the blockroot Merkle tree              |
| **action_mroot_savanna**                       | VARCHAR          | Alternative Merkle root for actions                        |
| **block_signing_key**                          | VARCHAR          | Signing key used for the block                            |
| **confirm_count**                              | ARRAY<NUMBER(10,0)> | Confirmation counts                                     |
| **size**                                       | NUMBER(38,0)     | Size of the block in bytes                                |
| **total_transactions**                         | NUMBER(38,0)     | Total number of transactions in the block                 |
| **successful_transactions**                    | NUMBER(38,0)     | Number of successful transactions                         |
| **failed_transactions**                        | NUMBER(38,0)     | Number of failed transactions                             |
| **total_actions**                              | NUMBER(38,0)     | Total number of actions in the block                      |
| **total_db_ops**                               | NUMBER(38,0)     | Total number of database operations                       |

</details>

### `transactions`

<details>
<summary>Click to expand Snowflake Table Schema for "transactions"</summary>

| Field Name                   | Type             | Description                                               |
|------------------------------|------------------|-----------------------------------------------------------|
| **block_time**               | TIMESTAMP_NTZ    | Timestamp of the block                                   |
| **block_number**             | NUMBER(38,0)     | Block number                                             |
| **block_hash**               | VARCHAR          | Block hash                                               |
| **block_date**               | DATE             | Date of the block                                        |
| **hash**                     | VARCHAR          | Transaction hash                                         |
| **index**                    | NUMBER(10,0)     | Transaction index within the block                       |
| **elapsed**                  | NUMBER(38,0)     | Elapsed time for the transaction                         |
| **net_usage**                | NUMBER(38,0)     | Network usage                                           |
| **scheduled**                | BOOLEAN          | Indicates if the transaction was scheduled                |
| **cpu_usage_micro_seconds**  | NUMBER(10,0)     | CPU usage in microseconds                                |
| **net_usage_words**          | NUMBER(10,0)     | Network usage in words                                   |
| **status**                   | VARCHAR          | Status of the transaction                                |
| **status_code**              | NUMBER(10,0)     | Numeric status code                                      |
| **success**                  | BOOLEAN          | Indicates if the transaction was successful              |
| **transaction_mroot**        | VARCHAR          | Merkle root of the transaction                            |
| **creator_action_indexes**   | ARRAY<NUMBER(10,0)> | Indexes of creator actions                             |
| **execution_action_indexes** | ARRAY<NUMBER(10,0)> | Indexes of execution actions                           |

</details>

### `actions`

<details>
<summary>Click to expand Snowflake Table Schema for "actions"</summary>

| Field Name                                         | Type             | Description                                               |
|----------------------------------------------------|------------------|-----------------------------------------------------------|
| **block_time**                                     | TIMESTAMP_NTZ    | Timestamp of the block                                   |
| **block_number**                                   | NUMBER(38,0)     | Block number                                             |
| **block_hash**                                     | VARCHAR          | Block hash                                               |
| **block_date**                                     | DATE             | Date of the block                                        |
| **tx_hash**                                        | VARCHAR          | Transaction hash                                         |
| **tx_success**                                     | BOOLEAN          | Indicates if the transaction was successful              |
| **abi_sequence**                                   | NUMBER(38,0)     | ABI sequence number                                      |
| **code_sequence**                                  | NUMBER(38,0)     | Code sequence number                                     |
| **digest**                                         | VARCHAR          | Digest of the action                                     |
| **global_sequence**                                | NUMBER(38,0)     | Global sequence number                                   |
| **receipt_receiver**                               | VARCHAR          | Receiver of the receipt                                  |
| **recv_sequence**                                  | NUMBER(38,0)     | Receive sequence                                         |
| **auth_sequence**                                  | ARRAY<NUMBER(38,0)> | Authorization sequences                                 |
| **auth_sequence_account_name**                     | ARRAY<VARCHAR>   | Account names for authorization sequences                |
| **account**                                        | VARCHAR          | Account executing the action                             |
| **name**                                           | VARCHAR          | Name of the action                                       |
| **json_data**                                      | VARCHAR          | JSON-formatted data of the action                        |
| **raw_data**                                       | VARCHAR          | Raw data of the action                                   |
| **index**                                          | NUMBER(10,0)     | Action index within the transaction                      |
| **action_ordinal**                                 | NUMBER(10,0)     | Ordinal position of the action                           |
| **receiver**                                       | VARCHAR          | Receiver of the action                                   |
| **context_free**                                   | BOOLEAN          | Indicates if the action is context-free                   |
| **elapsed**                                        | NUMBER(38,0)     | Elapsed time for the action                              |
| **console**                                        | VARCHAR          | Console output from the action                           |
| **raw_return_value**                               | VARCHAR          | Raw return value from the action                         |
| **json_return_value**                              | VARCHAR          | JSON-formatted return value from the action              |
| **creator_action_ordinal**                         | NUMBER(10,0)     | Ordinal of the creator action                            |
| **closest_unnotified_ancestor_action_ordinal**      | NUMBER(10,0)     | Ordinal of the closest unnotified ancestor action        |
| **action_mroot**                                   | VARCHAR          | Merkle root of the action                                 |
| **account_ram_deltas_account**                      | ARRAY<VARCHAR>   | Accounts affected by RAM deltas                           |
| **account_ram_deltas**                              | ARRAY<NUMBER(38,0)> | RAM delta values for accounts                          |

</details>

### `db_ops`

<details>
<summary>Click to expand Snowflake Table Schema for "db_ops"</summary>

| Field Name               | Type             | Description                                               |
|--------------------------|------------------|-----------------------------------------------------------|
| **block_time**           | TIMESTAMP_NTZ    | Timestamp of the block                                   |
| **block_number**         | NUMBER(38,0)     | Block number                                             |
| **block_hash**           | VARCHAR          | Block hash                                               |
| **block_date**           | DATE             | Date of the block                                        |
| **tx_hash**              | VARCHAR          | Transaction hash                                         |
| **tx_success**           | BOOLEAN          | Indicates if the transaction was successful              |
| **action_index**         | NUMBER(10,0)     | Index of the action within the transaction               |
| **index**                | NUMBER(10,0)     | Operation index                                          |
| **operation**            | VARCHAR          | Type of database operation                               |
| **operation_code**       | NUMBER(10,0)     | Numeric code representing the operation                  |
| **code**                 | VARCHAR          | Code associated with the operation                       |
| **scope**                | VARCHAR          | Scope of the operation                                   |
| **table_name**           | VARCHAR          | Name of the table involved in the operation              |
| **primary_key**          | VARCHAR          | Primary key affected by the operation                    |
| **old_payer**            | VARCHAR          | Previous payer before the operation                      |
| **new_payer**            | VARCHAR          | New payer after the operation                            |
| **old_data**             | VARCHAR          | Previous data before the operation                       |
| **new_data**             | VARCHAR          | New data after the operation                             |
| **old_data_json**        | VARCHAR          | Previous data in JSON format                             |
| **new_data_json**        | VARCHAR          | New data in JSON format                                   |

</details>

### `feature_ops`

<details>
<summary>Click to expand Snowflake Table Schema for "feature_ops"</summary>

| Field Name                | Type             | Description                                               |
|---------------------------|------------------|-----------------------------------------------------------|
| **block_time**            | TIMESTAMP_NTZ    | Timestamp of the block                                   |
| **block_number**          | NUMBER(38,0)     | Block number                                             |
| **block_hash**            | VARCHAR          | Block hash                                               |
| **block_date**            | DATE             | Date of the block                                        |
| **tx_hash**               | VARCHAR          | Transaction hash                                         |
| **tx_success**            | BOOLEAN          | Indicates if the transaction was successful              |
| **action_index**          | NUMBER(10,0)     | Index of the action within the transaction               |
| **feature_digest**        | VARCHAR          | Digest of the feature                                     |
| **kind**                  | VARCHAR          | Kind of feature operation                                 |
| **description_digest**    | VARCHAR          | Digest of the feature description                         |
| **protocol_feature_type** | VARCHAR          | Type of the protocol feature                              |

</details>

### `perm_ops`

<details>
<summary>Click to expand Snowflake Table Schema for "perm_ops"</summary>

| Field Name               | Type             | Description                                               |
|--------------------------|------------------|-----------------------------------------------------------|
| **block_time**           | TIMESTAMP_NTZ    | Timestamp of the block                                   |
| **block_number**         | NUMBER(38,0)     | Block number                                             |
| **block_hash**           | VARCHAR          | Block hash                                               |
| **block_date**           | DATE             | Date of the block                                        |
| **tx_hash**              | VARCHAR          | Transaction hash                                         |
| **tx_success**           | BOOLEAN          | Indicates if the transaction was successful              |
| **action_index**         | NUMBER(10,0)     | Index of the action within the transaction               |
| **operation**            | VARCHAR          | Type of permission operation                             |
| **operation_code**       | NUMBER(10,0)     | Numeric code representing the operation                  |
| **id**                   | NUMBER(38,0)     | Identifier for the permission operation                  |
| **parent_id**            | NUMBER(38,0)     | Parent identifier for hierarchical permissions           |
| **owner**                | VARCHAR          | Owner of the permission                                  |
| **name**                 | VARCHAR          | Name of the permission                                    |
| **threshold**            | NUMBER(10,0)     | Threshold required for the permission                    |
| **accounts**             | ARRAY<VARCHAR>   | Accounts associated with the permission                  |
| **accounts_weight**      | ARRAY<NUMBER(10,0)> | Weights of the associated accounts                  |
| **keys_public_key**      | ARRAY<VARCHAR>   | Public keys associated with the permission               |
| **keys_weight**          | ARRAY<NUMBER(10,0)> | Weights of the associated keys                      |
| **wait_sec**             | ARRAY<NUMBER(10,0)> | Wait times in seconds for delayed permissions        |
| **wait_weight**          | ARRAY<NUMBER(10,0)> | Weights of the wait times                             |

</details>

### `table_ops`

<details>
<summary>Click to expand Snowflake Table Schema for "table_ops"</summary>

| Field Name     | Type             | Description                                               |
|----------------|------------------|-----------------------------------------------------------|
| **block_time** | TIMESTAMP_NTZ    | Timestamp of the block                                   |
| **block_number**| NUMBER(38,0)     | Block number                                             |
| **block_hash** | VARCHAR          | Block hash                                               |
| **block_date** | DATE             | Date of the block                                        |
| **tx_hash**    | VARCHAR          | Transaction hash                                         |
| **tx_success** | BOOLEAN          | Indicates if the transaction was successful              |
| **action_index**| NUMBER(10,0)     | Index of the action within the transaction               |
| **index**       | NUMBER(10,0)     | Operation index                                          |
| **operation**   | VARCHAR          | Type of table operation                                  |
| **operation_code**| NUMBER(10,0)   | Numeric code representing the operation                  |
| **payer**       | VARCHAR          | Payer of the table operation                             |
| **code**        | VARCHAR          | Code associated with the table operation                 |
| **scope**       | VARCHAR          | Scope of the table operation                             |
| **table_name**  | VARCHAR          | Name of the table involved in the operation              |

</details>

### `ram_ops`

<details>
<summary>Click to expand Snowflake Table Schema for "ram_ops"</summary>

| Field Name          | Type             | Description                                               |
|---------------------|------------------|-----------------------------------------------------------|
| **block_time**      | TIMESTAMP_NTZ    | Timestamp of the block                                   |
| **block_number**    | NUMBER(38,0)     | Block number                                             |
| **block_hash**      | VARCHAR          | Block hash                                               |
| **block_date**      | DATE             | Date of the block                                        |
| **tx_hash**         | VARCHAR          | Transaction hash                                         |
| **tx_success**      | BOOLEAN          | Indicates if the transaction was successful              |
| **action_index**    | NUMBER(10,0)     | Index of the action within the transaction               |
| **operation**       | VARCHAR          | Type of RAM operation                                    |
| **operation_code**  | NUMBER(10,0)     | Numeric code representing the operation                  |
| **payer**           | VARCHAR          | Payer responsible for the RAM operation                  |
| **delta**           | NUMBER(38,0)     | Change in RAM usage                                      |
| **usage**           | NUMBER(38,0)     | Current RAM usage after the operation                    |
| **namespace**       | VARCHAR          | Namespace associated with the RAM operation              |
| **namespace_code**  | NUMBER(10,0)     | Code representing the namespace                          |
| **action**          | VARCHAR          | Action associated with the RAM operation                 |
| **action_code**     | NUMBER(10,0)     | Code representing the action                              |
| **unique_key**      | VARCHAR          | Unique key identifying the RAM operation                 |

</details>
