# `Solana`: Snowflake Datashare

## Data Dictionary

### `blocks`

<details>
<summary>Click to expand Snowflake Table Schema for "blocks"</summary>


| Column Name                      | Data Type        | Description                                                                     |
| -------------------------------- | ---------------- | ------------------------------------------------------------------------------- |
| time                             | TIMESTAMP_NTZ(3) | Timestamp indicating when this block was added to the blockchain.               |
| slot                             | NUMBER(38,0)     | Slot number of the block, representing its position in the Solana blockchain.   |
| height                           | NUMBER(38,0)     | Sequential number of the block in the blockchain.                               |
| hash                             | VARCHAR          | Unique identifier (hash) of the block.                                          |
| total_transactions               | NUMBER(10,0)     | Total number of transactions included in this block.                            |
| successful_transactions          | NUMBER(10,0)     | Number of transactions in the block that were successfully processed.           |
| failed_transactions              | NUMBER(10,0)     | Number of transactions in the block that failed during execution.               |
| total_vote_transactions          | NUMBER(10,0)     | Total number of vote transactions in this block.                                |
| total_non_vote_transactions      | NUMBER(10,0)     | Total number of non-vote transactions in this block.                            |
| successful_vote_transactions     | NUMBER(10,0)     | Number of vote transactions in this block that were successfully processed.     |
| successful_non_vote_transactions | NUMBER(10,0)     | Number of non-vote transactions in this block that were successfully processed. |
| failed_vote_transactions         |                  | Number of vote transactions in this block that failed during execution.         |
| failed_non_vote_transactions     | NUMBER(10,0)     | Number of non-vote transactions in this block that failed during execution.     |
| parent_slot                      | NUMBER(38,0)     | Slot number of the parent block that directly precedes this block in the chain. |
| previous_block_hash              | VARCHAR          | Hash of the previous block, linking this block to its predecessor.              |
| date                             | DATE             | Calendar date associated with this block.                                       |

</details>

### `rewards`

<details>
<summary>Click to expand Snowflake Table Schema for "rewards"</summary>

| Column Name               | Data Type        | Description                                                               |
| ------------------------- | ---------------- | ------------------------------------------------------------------------- |
| block_time                | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this reward was created.   |
| block_date                | DATE             | Calendar date associated with the block containing this reward.           |
| block_hash                | VARCHAR          | Unique identifier (hash) of the block containing this reward.             |
| block_slot                | NUMBER(38,0)     | Slot number of the block containing this reward.                          |
| block_height              | NUMBER(38,0)     | Sequential number of the block containing this reward.                    |
| block_previous_block_hash | VARCHAR          | Hash of the block immediately preceding the block containing this reward. |
| block_parent_slot         | NUMBER(38,0)     | Slot number of the parent block directly preceding this block.            |
| pubkey                    | VARCHAR          | Public key associated with the reward.                                    |
| lamports                  | NUMBER(38,0)     | Amount of lamports earned as a reward.                                    |
| pre_balance               | NUMBER(38,0)     | Account balance in lamports before the reward was applied.                |
| post_balance              | NUMBER(38,0)     | Account balance in lamports after the reward was applied.                 |
| reward_type               | VARCHAR          | Type of reward earned (e.g., voting, staking).                            |
| commission                | VARCHAR          | Commission percentage applied to the reward, if applicable.               |

</details>

### `transactions` & `vote_transactions`

<details>
<summary>Click to expand Snowflake Table Schema for "transactions" and "vote_transactions" </summary>


| Column Name                | Data Type           | Description                                                                    |
| -------------------------- | ------------------- | ------------------------------------------------------------------------------ |
| block_time                 | TIMESTAMP_NTZ(3)    | Timestamp indicating when the block containing this transaction was created.   |
| block_hash                 | VARCHAR             | Unique identifier (hash) of the block containing this transaction.             |
| block_date                 | DATE                | Calendar date associated with the block containing this transaction.           |
| block_slot                 | NUMBER(38,0)        | Slot number of the block containing this transaction.                          |
| block_height               | NUMBER(38,0)        | Sequential number of the block containing this transaction.                    |
| block_previous_block_hash  | VARCHAR             | Hash of the block immediately preceding the block containing this transaction. |
| block_parent_slot          | NUMBER(38,0)        | Slot number of the parent block directly preceding this block.                 |
| id                         | VARCHAR             | Unique identifier (hash) of the transaction.                                   |
| index                      | NUMBER(10,0)        | Index of the transaction within the block.                                     |
| fee                        | NUMBER(38,0)        | Transaction fee paid in lamports.                                              |
| required_signatures        | NUMBER(10,0)        | Number of required signatures for the transaction.                             |
| required_signed_accounts   | NUMBER(10,0)        | Number of accounts required to sign the transaction.                           |
| required_unsigned_accounts | NUMBER(10,0)        | Number of accounts not required to sign the transaction.                       |
| signature                  | VARCHAR             | Main signature of the transaction.                                             |
| success                    | BOOLEAN             | Indicates whether the transaction was successfully executed.                   |
| error                      | VARCHAR             | Error message if the transaction failed.                                       |
| recent_block_hash          | VARCHAR             | Hash of a recent block used to validate the transaction.                       |
| account_keys               | ARRAY<VARCHAR>      | List of account public keys involved in the transaction.                       |
| log_messages               | ARRAY<VARCHAR>      | List of log messages generated during transaction execution.                   |
| pre_balances               | ARRAY<NUMBER(38,0)> | List of account balances in lamports before the transaction.                   |
| post_balances              | ARRAY<NUMBER(38,0)> | List of account balances in lamports after the transaction.                    |
| signatures                 | ARRAY<VARCHAR>      | List of signatures included in the transaction.                                |
| signer                     | VARCHAR             | Primary signer of the transaction.                                             |
| signers                    | ARRAY<VARCHAR>      | List of all signers involved in the transaction.                               |

</details>

### `instruction_calls` & `vote_instructions_calls`

<details>
<summary>Click to expand Snowflake Table Schema for "instruction_calls" and "vote_instruction_calls"</summary>

| Column Name               | Data Type        | Description                                                                              |
| ------------------------- | ---------------- | ---------------------------------------------------------------------------------------- |
| block_time                | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this instruction was created.             |
| block_hash                | VARCHAR          | Unique identifier (hash) of the block containing this instruction.                       |
| block_date                | DATE             | Calendar date associated with the block containing this instruction.                     |
| block_slot                | NUMBER(38,0)     | Slot number of the block containing this instruction.                                    |
| block_height              | NUMBER(38,0)     | Sequential number of the block containing this instruction.                              |
| block_previous_block_hash | VARCHAR          | Hash of the block immediately preceding the block containing this instruction.           |
| block_parent_slot         | NUMBER(38,0)     | Slot number of the parent block directly preceding this block.                           |
| tx_id                     | VARCHAR          | Unique identifier (hash) of the transaction containing this instruction.                 |
| tx_index                  | NUMBER(10,0)     | Index of the transaction within the block.                                               |
| tx_signer                 | VARCHAR          | Primary signer of the transaction containing this instruction.                           |
| tx_success                | BOOLEAN          | Indicates whether the transaction containing this instruction was successfully executed. |
| log_messages              | ARRAY<VARCHAR>   | List of log messages generated during execution of this instruction.                     |
| outer_instruction_index   | NUMBER(10,0)     | Index of the outer instruction in the transaction.                                       |
| inner_instruction_index   | NUMBER(10,0)     | Index of the inner instruction within the outer instruction, if applicable.              |
| inner_executing_account   | VARCHAR          | Account executing the inner instruction, if applicable.                                  |
| outer_executing_account   | VARCHAR          | Account executing the outer instruction.                                                 |
| executing_account         | VARCHAR          | Account executing the instruction (outer or inner).                                      |
| is_inner                  | BOOLEAN          | Indicates whether this is an inner instruction.                                          |
| data                      | VARCHAR          | Data associated with the instruction, typically in binary or encoded format.             |
| account_arguments         | ARRAY<VARCHAR>   | List of accounts used as arguments for the instruction.                                  |
| inner_instructions        | VARCHAR          | Serialized representation of inner instructions (if any).                                |

</details>

### `account_activity` & `vote_account_activity` 

<details>
<summary>Click to expand Snowflake Table Schema for "account_activity" and "vote_account_activity"</summary>

| Column Name               | Data Type        | Description                                                                 |
| ------------------------- | ---------------- | --------------------------------------------------------------------------- |
| block_time                | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this activity was created.   |
| block_hash                | VARCHAR          | Unique identifier (hash) of the block containing this activity.             |
| block_date                | DATE             | Calendar date associated with the block containing this activity.           |
| block_slot                | NUMBER(38,0)     | Slot number of the block containing this activity.                          |
| block_height              | NUMBER(38,0)     | Sequential number of the block containing this activity.                    |
| block_previous_block_hash | VARCHAR          | Hash of the block immediately preceding the block containing this activity. |
| block_parent_slot         | NUMBER(38,0)     | Slot number of the parent block directly preceding this block.              |
| address                   | VARCHAR          | Public address of the account involved in this activity.                    |
| tx_index                  | INTEGER          | Index of the transaction within the block.                                  |
| tx_id                     | VARCHAR          | Unique identifier (hash) of the transaction involving this activity.        |
| tx_success                | BOOLEAN          | Indicates whether the transaction was successfully executed.                |
| signed                    | BOOLEAN          | Indicates whether the account signed the transaction.                       |
| writable                  | BOOLEAN          | Indicates whether the account was writable during the transaction.          |
| token_mint_address        | VARCHAR          | Address of the token mint associated with this activity, if applicable.     |
| pre_balance               | NUMBER(38,0)     | Account balance in lamports before the transaction.                         |
| post_balance              | NUMBER(38,0)     | Account balance in lamports after the transaction.                          |
| balance_change            | NUMBER(38,0)     | Change in lamport balance as a result of the transaction.                   |
| pre_token_balance         | NUMBER(38,18)    | Token balance before the transaction, if applicable.                        |
| post_token_balance        | NUMBER(38,18)    | Token balance after the transaction, if applicable.                         |
| token_balance_change      | NUMBER(38,18)    | Change in token balance as a result of the transaction, if applicable.      |
| token_balance_owner       | VARCHAR          | Owner of the token balance associated with this activity, if applicable.    |

</details>