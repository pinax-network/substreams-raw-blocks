# `Solana`: Snowflake Datashare

This dataset offers a detailed view of Solana based blockchain activity, providing critical data points across blocks, transactions, traces, and logs for comprehensive blockchain analysis. Designed for developers, analysts, and researchers, this dataset supports advanced analytics, indexing, and querying capabilities tailored to Solana's unique structure and behavior.

Free access to the dataset on the [Snowflake Data Marketplace](https://app.snowflake.com/marketplace).

## Tables/Views

| Data Type                 | Description                                                                                 |
|---------------------------|---------------------------------------------------------------------------------------------|
| `blocks`                  | Block-level data for Solana, including time, slot, height, hashes, and transaction counts. |
| `rewards`                 | Reward details for validators, including lamports, balances, and commission information.    |
| `transactions`            | Transaction details such as signatures, account keys, balances, and compute units consumed. |
| `instruction_calls`       | Detailed information on instruction calls including execution context and metadata.         |
| `account_activity`        | Records of account activities, balance changes, and token-related information.              |
| `vote_transactions`       | Vote transaction details, structured the same as standard transactions but specific to votes. |
| `vote_instruction_calls`  | Instruction calls associated with vote transactions, sharing the same structure as `instruction_calls`. |
| `vote_account_activity`   | Account activity related to voting, structured similarly to general account activity.        |

## Sample SQL Queries

**Daily Active Users (DAU)**

```sql
SELECT
    block_date,
    count(distinct address) AS user
FROM solana.account_activity
GROUP BY block_date
ORDER BY block_date ASC;
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

**Tokens**

```sql
SELECT
  a.tx_id,
  a.address,
  a.token_mint_address,
  a.pre_token_balance,
  a.post_token_balance,
  a.token_balance_change,
  a.block_time
FROM
  solana.account_activity a
WHERE
  a.token_mint_address IS NOT NULL
ORDER BY
  a.block_time DESC
LIMIT
  100;
```


**View the first and last block indexed**
> This query tells you how fresh the data is.

```sql
SELECT
  MIN(number) AS "First block",
  MAX(number) AS "Newest block",
  COUNT(1) AS "Total number of blocks"
FROM
  solana.blocks;
```

## Data Dictionary

### `blocks`

<details>
<summary>Click to expand Snowflake Table Schema for "blocks"</summary>

| Field Name                    | Type             | Description                                               |
|-------------------------------|------------------|-----------------------------------------------------------|
| **time**                      | TIMESTAMP_NTZ(3) | Timestamp of the block                                    |
| **date**                      | DATE             | Date of the block                                         |
| **hash**                      | VARCHAR          | Block hash                                                |
| **slot**                      | NUMBER(38,0)     | Block slot number                                         |
| **height**                    | NUMBER(38,0)     | Block height                                              |
| **previous_block_hash**       | VARCHAR          | Hash of the previous block                                |
| **parent_slot**               | NUMBER(38,0)     | Parent slot number                                        |
| **total_transactions**        | NUMBER(38,0)     | Total transactions in the block                           |
| **successful_transactions**   | NUMBER(38,0)     | Number of successful transactions                         |
| **failed_transactions**       | NUMBER(38,0)     | Number of failed transactions                             |
| **total_vote_transactions**   | NUMBER(38,0)     | Total vote transactions in the block                      |
| **total_non_vote_transactions** | NUMBER(38,0)   | Total non-vote transactions in the block                  |
| **successful_vote_transactions** | NUMBER(38,0)  | Number of successful vote transactions                    |
| **successful_non_vote_transactions** | NUMBER(38,0)| Number of successful non-vote transactions               |
| **failed_vote_transactions**  | NUMBER(38,0)     | Number of failed vote transactions                        |
| **failed_non_vote_transactions** | NUMBER(38,0)   | Number of failed non-vote transactions                    |

</details>

### `rewards`

<details>
<summary>Click to expand Snowflake Table Schema for "rewards"</summary>

| Field Name                 | Type             | Description                                                    |
|----------------------------|------------------|----------------------------------------------------------------|
| **block_time**             | TIMESTAMP_NTZ(3) | Timestamp of the block when reward was issued                  |
| **block_date**             | DATE             | Date of the block                                              |
| **block_hash**             | VARCHAR          | Hash of the block                                              |
| **block_slot**             | NUMBER(38,0)     | Slot of the block                                              |
| **block_height**           | NUMBER(38,0)     | Height of the block                                            |
| **block_previous_block_hash** | VARCHAR       | Hash of the previous block                                     |
| **block_parent_slot**      | NUMBER(38,0)     | Parent slot number of the block                                |
| **pubkey**                 | VARCHAR          | Public key of the rewarded entity                              |
| **lamports**               | NUMBER(38,0)     | Amount of lamports rewarded                                    |
| **pre_balance**            | NUMBER(38,0)     | Balance before reward                                          |
| **post_balance**           | NUMBER(38,0)     | Balance after reward                                           |
| **reward_type**            | VARCHAR          | Type of reward                                                 |
| **commission**             | VARCHAR          | Commission rate or details                                     |

</details>

### `transactions`

<details>
<summary>Click to expand Snowflake Table Schema for "transactions"</summary>

| Field Name                  | Type               | Description                                                                   |
|-----------------------------|--------------------|-------------------------------------------------------------------------------|
| **block_time**              | TIMESTAMP_NTZ(3)   | Timestamp of the block                                                        |
| **block_hash**              | VARCHAR            | Hash of the block                                                             |
| **block_date**              | DATE               | Date of the block                                                             |
| **block_slot**              | NUMBER(38,0)       | Slot of the block                                                             |
| **block_height**            | NUMBER(38,0)       | Height of the block                                                           |
| **block_previous_block_hash** | VARCHAR          | Hash of the previous block                                                    |
| **block_parent_slot**       | NUMBER(38,0)       | Parent slot number of the block                                               |
| **id**                      | VARCHAR            | Transaction ID                                                                |
| **index**                   | NUMBER(10,0)       | Index of the transaction within the block                                     |
| **required_signatures**     | NUMBER(10,0)       | Number of required signatures                                                 |
| **required_signed_accounts**| NUMBER(10,0)       | Number of required signed accounts                                            |
| **required_unsigned_accounts** | NUMBER(10,0)    | Number of required unsigned accounts                                          |
| **signature**               | VARCHAR            | Primary signature of the transaction                                          |
| **signatures**              | ARRAY<VARCHAR>     | List of signatures                                                              |
| **recent_block_hash**       | VARCHAR            | Recent block hash used in transaction message                                 |
| **account_keys**            | ARRAY<VARCHAR>     | Account keys involved in the transaction                                      |
| **log_messages**            | ARRAY<VARCHAR>     | Log messages generated during transaction execution                           |
| **pre_balances**            | ARRAY<NUMBER(38,0)> | Balances before transaction (corresponding to account_keys)                  |
| **post_balances**           | ARRAY<NUMBER(38,0)> | Balances after transaction (corresponding to account_keys)                   |
| **signer**                  | VARCHAR            | Primary signer of the transaction                                             |
| **signers**                 | ARRAY<VARCHAR>     | List of signers                                                                 |
| **fee**                     | NUMBER(38,0)       | Fee paid for the transaction                                                  |
| **success**                 | BOOLEAN            | Whether the transaction was successful                                        |
| **error**                   | VARCHAR            | Error message if the transaction failed                                       |
| **compute_units_consumed**  | NUMBER(38,0)       | Total compute units consumed by the transaction                               |

</details>

### `instruction_calls`

<details>
<summary>Click to expand Snowflake Table Schema for "instruction_calls"</summary>

| Field Name                      | Type               | Description                                                                    |
|---------------------------------|--------------------|--------------------------------------------------------------------------------|
| **block_time**                  | TIMESTAMP_NTZ(3)   | Timestamp of the block                                                         |
| **block_hash**                  | VARCHAR            | Hash of the block                                                              |
| **block_date**                  | DATE               | Date of the block                                                              |
| **block_slot**                  | NUMBER(38,0)       | Slot of the block                                                              |
| **block_height**                | NUMBER(38,0)       | Height of the block                                                            |
| **block_previous_block_hash**   | VARCHAR            | Hash of the previous block                                                     |
| **block_parent_slot**           | NUMBER(38,0)       | Parent slot number of the block                                                |
| **tx_id**                       | VARCHAR            | ID of the transaction                                                          |
| **tx_index**                    | NUMBER(10,0)       | Index of the transaction within the block                                      |
| **tx_signer**                   | VARCHAR            | Signer of the transaction                                                      |
| **tx_success**                  | BOOLEAN            | Indicates if the transaction succeeded                                         |
| **log_messages**                  | ARRAY<VARCHAR>   | Log messages for the transaction                                               |
| **outer_instruction_index**     | NUMBER(10,0)       | Index of the outer instruction                                                 |
| **inner_instruction_index**     | NUMBER(10,0)       | Index of the inner instruction                                                 |
| **inner_executing_account**     | VARCHAR            | Account executing the inner instruction                                        |
| **outer_executing_account**     | VARCHAR            | Account executing the outer instruction                                        |
| **executing_account**           | VARCHAR            | Account executing the instruction                                              |
| **is_inner**                    | BOOLEAN            | Flag indicating if this is an inner instruction                                |
| **data**                        | VARCHAR            | Data payload of the instruction                                                |
| **account_arguments**           | ARRAY<VARCHAR>     | Account arguments for the instruction                                          |
| **inner_instructions**          | VARCHAR            | Inner instructions as a serialized string (if applicable)                        |
| **program_id**                  | VARCHAR            | Program ID for the instruction                                                 |
| **stack_height**                | NUMBER(10,0)       | Stack height of the instruction                                                |
| **compute_units_consumed**      | NUMBER(38,0)       | Compute units consumed by the instruction                                      |
| **fee**                         | NUMBER(38,0)       | Fee associated with the instruction                                            |

</details>

### `account_activity`

<details>
<summary>Click to expand Snowflake Table Schema for "account_activity"</summary>

| Field Name               | Type               | Description                                                                      |
|--------------------------|--------------------|----------------------------------------------------------------------------------|
| **block_time**           | TIMESTAMP_NTZ(3)   | Timestamp of the block                                                           |
| **block_hash**           | VARCHAR            | Hash of the block                                                                |
| **block_date**           | DATE               | Date of the block                                                                |
| **block_slot**           | NUMBER(38,0)       | Slot of the block                                                                |
| **block_height**         | NUMBER(38,0)       | Height of the block                                                              |
| **block_previous_block_hash** | VARCHAR       | Hash of the previous block                                                       |
| **block_parent_slot**    | NUMBER(38,0)       | Parent slot number of the block                                                  |
| **address**              | VARCHAR            | Account address                                                                  |
| **tx_index**             | NUMBER(10,0)       | Transaction index within the block                                               |
| **tx_id**                | VARCHAR            | ID of the transaction                                                            |
| **tx_success**           | BOOLEAN            | Indicates if the transaction was successful                                      |
| **signed**               | BOOLEAN            | Whether the account signed the transaction                                       |
| **writable**             | BOOLEAN            | Whether the account was writable during the transaction                            |
| **token_mint_address**   | VARCHAR            | (Optional) Token mint address associated with the account activity               |
| **pre_balance**          | NUMBER(38,0)       | Balance before the transaction                                                   |
| **post_balance**         | NUMBER(38,0)       | Balance after the transaction                                                    |
| **balance_change**       | NUMBER(38,0)       | Change in balance                                                                |
| **pre_token_balance**    | DOUBLE             | (Optional) Token balance before the transaction                                  |
| **post_token_balance**   | DOUBLE             | (Optional) Token balance after the transaction                                   |
| **token_balance_change** | DOUBLE             | (Optional) Change in token balance                                               |
| **token_balance_owner**  | VARCHAR            | (Optional) Owner of the token balance                                            |

</details>

### `vote_transactions`

<details>
<summary>Click to expand Snowflake Table Schema for "vote_transactions"</summary>

**Note:** The schema for `vote_transactions` is identical to the `transactions` table.

</details>

### `vote_instruction_calls`

<details>
<summary>Click to expand Snowflake Table Schema for "vote_instruction_calls"</summary>

**Note:** The schema for `vote_instruction_calls` is identical to the `instruction_calls` table.

</details>

### `vote_account_activity`

<details>
<summary>Click to expand Snowflake Table Schema for "vote_account_activity"</summary>

**Note:** The schema for `vote_account_activity` is identical to the `account_activity` table.

</details>
