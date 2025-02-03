# `EVM`: Snowflake Datashare

> EVM-compatible blockchain data on Snowflake.

## Supported chains

- [x] **Ethereum**
- [x] **Base**
- [x] **BSC** (Binance Smart Chain)

This dataset offers a detailed view of EVM-based blockchain activity, providing critical data points across blocks, transactions, traces, and logs for comprehensive blockchain analysis. Designed for developers, analysts, and researchers, this dataset supports advanced analytics, indexing, and querying capabilities tailored to the EVM’s unique structure and behavior.

Try our datasets for free (with limited historical coverage) on the [Snowflake Data Marketplace](https://app.snowflake.com/marketplace).

## Tables/Views

| Data Type           | Description |
|---------------------|-------------|
| `blocks`            | Block-level data, including height, timestamp, and miner. |
| `transactions`      | Transaction details like sender, recipient, and gas fees. |
| `traces`            | Internal transactions and contract interactions. |
| `logs`              | Event logs from smart contracts. |
| `storage_changes`   | Changes in smart contract storage. |
| `code_changes`      | Smart contract code updates. |
| `creation_traces`   | Contract creation transactions. |

## Sample SQL Queries

**Active Users**

```sql
SELECT
    block_date,
    count(distinct "from") AS user
FROM transactions
GROUP BY block_date
ORDER BY block_date ASC
```

**Top Contracts**

```sql
SELECT
    "to" AS contract,
    count(*) AS transactions
FROM transactions
GROUP BY contract
ORDER BY transactions DESC
LIMIT 10
```

**ERC-20 Transfers**

```sql
SELECT
    block_date,
    count(*) as total
FROM
    traces
WHERE
    tx_success = true AND
    SUBSTR(input, 1, 10) IN ('0xa9059cbb', '0x23b872dd') // Transfer and TransferFrom
GROUP BY block_date
ORDER BY block_date;
```

**View the first and last block indexed**

> This query tells you how fresh the data is.

```sql
SELECT
  MIN(number) AS "First block",
  MAX(number) AS "Newest block",
  MIN(time) AS "First time",
  MAX(time) AS "Newest time",
  1/ (COUNT(1) / (DATE_PART(EPOCH_SECOND, MAX(time)) - DATE_PART(EPOCH_SECOND, MIN(time)))) AS "Blocks/second",
  COUNT(1) AS "Total number of blocks"
FROM
  blocks;
```

## Data Dictionary

### `blocks`

<details>
<summary>Click to expand Snowflake Table Schema for "blocks"</summary>

| Field Name               | Type             | Description                                            |
|--------------------------|------------------|--------------------------------------------------------|
| **time**                 | TIMESTAMP_NTZ(3) |                                                        |
| **number**               | NUMBER(38,0)     |                                                        |
| **date**                 | DATE             |                                                        |
| **hash**                 | VARCHAR          | EVM Hash                                               |
| **parent_hash**          | VARCHAR          | EVM Hash                                               |
| **nonce**                | NUMBER(38,0)     |                                                        |
| **ommers_hash**          | VARCHAR          | EVM Hash                                               |
| **logs_bloom**           | VARCHAR          |                                                        |
| **transactions_root**    | VARCHAR          | EVM Hash                                               |
| **state_root**           | VARCHAR          | EVM Hash                                               |
| **receipts_root**        | VARCHAR          | EVM Hash                                               |
| **withdrawals_root**     | VARCHAR          | EVM Root EIP-4895 (Shanghai Fork).          |
| **parent_beacon_root**   | VARCHAR          | EVM Root EIP-4788 (Dencun Fork).            |
| **miner**                | VARCHAR          | EVM Address                                            |
| **difficulty**           | NUMBER(38,0)     |                                              |
| **total_difficulty**     | VARCHAR          | UInt256.                                    |
| **mix_hash**             | VARCHAR          | EVM Hash                                               |
| **extra_data**           | VARCHAR          |                                                        |
| **extra_data_utf8**      | VARCHAR          |                                                        |
| **gas_limit**            | NUMBER(38,0)     |                                                        |
| **gas_used**             | NUMBER(38,0)     |                                                        |
| **base_fee_per_gas**     | VARCHAR          | EIP-1559 (London Fork).                    |
| **blob_gas_used**        | VARCHAR          | EIP-4844 (Dencun Fork).                    |
| **excess_blob_gas**      | VARCHAR          | EIP-4844 (Dencun Fork).                    |
| **size**                 | NUMBER(38,0)     | Block size in bytes                                    |
| **total_transactions**   | NUMBER(38,0)     |                                                        |
| **successful_transactions** | NUMBER(38,0)  |                                                        |
| **failed_transactions**  | NUMBER(38,0)     |                                                        |
| **total_balance_changes** | NUMBER(38,0)    |                                                        |
| **total_withdrawals**    | NUMBER(38,0)     |                                                        |
| **detail_level**         | VARCHAR          |                                                        |
| **detail_level_code**    | NUMBER(10,0)     |                                                        |

</details>

### `transactions`

<details>
  <summary>Click to expand Snowflake Table Schema for "transactions"</summary>

  | Field Name                   | Type                       | Description                  |
  |------------------------------|----------------------------|------------------------------|
  | block_time                   | TIMESTAMP_NTZ              | Timestamp of the block       |
  | block_number                 | NUMBER(38,0)               | Block number                 |
  | block_hash                   | STRING                     | EVM Hash                     |
  | block_date                   | DATE                       | Date of the block            |
  | transactions_root            | STRING                     | EVM Hash                     |
  | receipts_root                | STRING                     | EVM Hash                     |
  | index                      | NUMBER(10,0)               | Transaction index            |
  | hash                         | STRING                     | EVM Hash                     |
  | "from"                         | STRING                     | EVM Address                  |
  | "to"                           | STRING                     | EVM Address                  |
  | nonce                        | NUMBER(38,0)               | Nonce                        |
  | status                       | STRING                     | Status                       |
  | status_code                  | NUMBER(10,0)               | Status code                  |
  | success                      | BOOLEAN                    | Transaction success indicator|
  | gas_price                    | STRING                     | UInt256                      |
  | gas_limit                    | NUMBER(38,0)               | Gas limit                    |
  | value                        | STRING                     | UInt256                      |
  | data                         | STRING                     | Transaction data             |
  | v                            | STRING                     |                              |
  | r                            | STRING                     | EVM Hash                     |
  | s                            | STRING                     | EVM Hash                     |
  | gas_used                     | NUMBER(38,0)               | Gas used                     |
  | type                         | STRING                     | EIP-1559 Type                |
  | type_code                    | NUMBER(10,0)               | EIP-1559 Type code           |
  | max_fee_per_gas              | STRING                     | UInt256                      |
  | max_priority_fee_per_gas     | STRING                     | UInt256                      |
  | begin_ordinal                | NUMBER(38,0)               | Begin ordinal                |
  | end_ordinal                  | NUMBER(38,0)               | End ordinal                  |
  | blob_gas_price               | STRING                     | UInt256                      |
  | blob_gas_used                | NUMBER(38,0)               | Blob gas used                |
  | cumulative_gas_used          | NUMBER(38,0)               | Cumulative gas used          |
  | logs_bloom                   | STRING                     | Logs bloom filter            |
  | state_root                   | STRING                     | EVM Hash                     |

</details>

### `traces`

<details>
  <summary>Click to expand Snowflake Table Schema for "traces"</summary>

  | Field Name                   | Type                       | Description                                               |
  |------------------------------|----------------------------|-----------------------------------------------------------|
  | block_time                   | TIMESTAMP_NTZ              | Timestamp of the block                                    |
  | block_number                 | NUMBER(38,0)               | Block number                                              |
  | block_hash                   | STRING                     | EVM Hash                                                  |
  | block_date                   | DATE                       | Date of the block                                         |
  | tx_hash                      | STRING                     | EVM Hash of the transaction                               |
  | tx_index                     | NUMBER(10,0)               | Transaction index                                         |
  | tx_status                    | STRING                     | Status of the transaction                                 |
  | tx_status_code               | NUMBER(10,0)               | Status code of the transaction                            |
  | tx_success                   | BOOLEAN                    | Indicates if the transaction was successful               |
  | from                         | STRING                     | EVM Address of the sender                                 |
  | to                           | STRING                     | EVM Address of the receiver                               |
  | `index`                      | NUMBER(10,0)               | Trace index                                               |
  | parent_index                 | NUMBER(10,0)               | Parent trace index                                        |
  | depth                        | NUMBER(10,0)               | Trace depth                                               |
  | caller                       | STRING                     | EVM Address of the caller                                 |
  | call_type                    | STRING                     | Type of call                                              |
  | call_type_code               | NUMBER(10,0)               | Code for the call type                                    |
  | address                      | STRING                     | EVM Address involved in the trace                         |
  | value                        | STRING                     | UInt256                                                   |
  | gas_limit                    | NUMBER(38,0)               | Gas limit for the trace                                   |
  | gas_consumed                 | NUMBER(38,0)               | Gas consumed by the trace                                 |
  | return_data                  | STRING                     | Data returned by contract calls (RETURN or REVERT)        |
  | input                        | STRING                     | Input data for the trace                                  |
  | suicide                      | BOOLEAN                    | Indicates if a self-destruct occurred                     |
  | failure_reason               | STRING                     | Reason for failure, if any                                |
  | state_reverted               | BOOLEAN                    | Indicates if state was reverted                           |
  | status_reverted              | BOOLEAN                    | Indicates if status was reverted                          |
  | status_failed                | BOOLEAN                    | Indicates if status failed                                |
  | executed_code                | BOOLEAN                    | Indicates if code was executed                            |
  | begin_ordinal                | NUMBER(38,0)               | Begin ordinal                                             |
  | end_ordinal                  | NUMBER(38,0)               | End ordinal                                               |

</details>

### `logs`

<details>
  <summary>Click to expand Snowflake Table Schema for "logs"</summary>

  | Field Name                   | Type                       | Description                                               |
  |------------------------------|----------------------------|-----------------------------------------------------------|
  | block_time                   | TIMESTAMP_NTZ              | Timestamp of the block                                    |
  | block_number                 | NUMBER(38,0)               | Block number                                              |
  | block_hash                   | STRING                     | EVM Hash                                                  |
  | block_date                   | DATE                       | Date of the block                                         |
  | tx_hash                      | STRING                     | EVM Hash of the transaction                               |
  | tx_index                     | NUMBER(10,0)               | Transaction index                                         |
  | tx_status                    | STRING                     | Status of the transaction                                 |
  | tx_status_code               | NUMBER(10,0)               | Status code of the transaction                            |
  | tx_success                   | BOOLEAN                    | Indicates if the transaction was successful               |
  | tx_from                      | STRING                     | EVM Address of the sender                                 |
  | tx_to                        | STRING                     | EVM Address of the receiver                               |
  | `index`                      | NUMBER(10,0)               | Log index                                                 |
  | block_index                  | NUMBER(10,0)               | Block index                                               |
  | contract_address             | STRING                     | EVM Address of the contract                               |
  | topic0                       | STRING                     | Primary topic (EVM Hash)                                  |
  | topic1                       | STRING                     | Secondary topic (EVM Hash)                                |
  | topic2                       | STRING                     | Tertiary topic (EVM Hash)                                 |
  | topic3                       | STRING                     | Quaternary topic (EVM Hash)                               |
  | data                         | STRING                     | Log data                                                  |

</details>

### `storage_changes`

<details>
<summary>Click to expand Snowflake Table Schema for "storage_changes"</summary>

  | Field Name                   | Type                       | Description                    |
  |------------------------------|----------------------------|--------------------------------|
  | block_time                   | TIMESTAMP_NTZ              | Timestamp of the block         |
  | block_number                 | NUMBER(38,0)               | Block number                   |
  | block_hash                   | STRING                     | EVM Hash                       |
  | block_date                   | DATE                       | Date of the block              |
  | ordinal                      | NUMBER(38,0)               | Block global ordinal           |
  | address                      | STRING                     | EVM Address                    |
  | key                          | STRING                     | EVM Hash                       |
  | new_value                    | STRING                     | New storage value (EVM Hash)   |
  | old_value                    | STRING                     | Old storage value (EVM Hash)   |

</details>

### `code_changes`

<details>
<summary>Click to expand Snowflake Table Schema for "code_changes"</summary>

  | Field Name                   | Type                       | Description                    |
  |------------------------------|----------------------------|--------------------------------|
  | block_time                   | TIMESTAMP_NTZ              | Timestamp of the block         |
  | block_number                 | NUMBER(38,0)               | Block number                   |
  | block_hash                   | STRING                     | EVM Hash                       |
  | block_date                   | DATE                       | Date of the block              |
  | ordinal                      | NUMBER(38,0)               | Block global ordinal           |
  | address                      | STRING                     | EVM Address                    |
  | old_hash                     | STRING                     | Old code hash (EVM Hash)       |
  | old_code                     | STRING                     | Old code                       |
  | new_hash                     | STRING                     | New code hash (EVM Hash)       |
  | new_code                     | STRING                     | New code                       |

</details>
