# `Arweave`: Snowflake Datashare

## Data Dictionary

### `blocks`

<details>
<summary>Click to expand Snowflake Table Schema for "blocks"</summary>

| Column Name      | Data Type        | Description                                                                                      |
| ---------------- | ---------------- | ------------------------------------------------------------------------------------------------ |
| time             | TIMESTAMP_NTZ(3) | Timestamp indicating when the block was created                                                  |
| height           | NUMBER(38,0)     | Height of the block in the Arweave chain                                                         |
| date             | DATE             | Calendar date associated with the block                                                          |
| indep_hash       | VARCHAR          | Unique `indep_hash` identifier of the block                                                      |
| nonce            | BINARY           | Nonce chosen to solve the mining problem for this block                                          |
| previous_block   | VARCHAR          | `indep_hash` of the previous block in the chain                                                  |
| timestamp        | NUMBER(38,0)     | POSIX time of block discovery                                                                    |
| last_retarget    | NUMBER(38,0)     | POSIX time of the last mining difficulty retarget                                                |
| diff             | BINARY           | Mining difficulty threshold; block's `hash` must be greater than this value                      |
| hash             | VARCHAR          | Hash of the block satisfying the mining difficulty                                               |
| tx_root          | VARCHAR          | Merkle root of the tree of Merkle roots of the block's transaction data                          |
| wallet_list      | VARCHAR          | Root hash of the Merkle Patricia Tree for wallet balances and last transaction identifiers       |
| reward_addr      | VARCHAR          | Address of the account receiving the block rewards (or null byte if unclaimed)                   |
| reward_pool      | BINARY           | Size of the reward pool in bytes                                                                 |
| weave_size       | BINARY           | Total size of the Arweave weave in bytes                                                         |
| block_size       | BINARY           | Size of this block in bytes                                                                      |
| cumulative_diff  | BINARY           | Sum of the average number of hashes computed by the network for past blocks, including this one  |
| hash_list_merkle | VARCHAR          | Merkle root of the block index, containing triplets of `indep_hash`, `weave_size`, and `tx_root` |
| poa_option       | VARCHAR          | Proof of access option (required after version 24; default otherwise)                            |
| poa_tx_path      | VARCHAR          | Proof of access transaction path (required after version 24; default otherwise)                  |
| poa_data_path    | VARCHAR          | Proof of access data path (required after version 24; default otherwise)                         |
| poa_chunk        | VARCHAR          | Proof of access chunk (required after version 24; default otherwise)                             |

</details>

### `transactions`

<details>
<summary>Click to expand Snowflake Table Schema for "transactions"</summary>

| Column Name      | Data Type        | Description                                                                          |
| ---------------- | ---------------- | ------------------------------------------------------------------------------------ |
| block_time       | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this transaction was created          |
| block_height     | NUMBER(38,0)     | Height of the block in the Arweave chain containing this transaction                 |
| block_date       | DATE             | Calendar date associated with the block containing this transaction                  |
| block_indep_hash | VARCHAR          | Unique `indep_hash` of the block containing this transaction                         |
| format           | NUMBER(10,0)     | Format version of the transaction                                                    |
| id               | VARCHAR          | Unique identifier of the transaction                                                 |
| index            | NUMBER(10,0)     | Index of the transaction in the block's transaction list                             |
| last_tx          | VARCHAR          | Identifier of the previous transaction from the same wallet or a recent block anchor |
| owner            | VARCHAR          | Public key of the transaction's sender                                               |
| target           | VARCHAR          | Address of the recipient (SHA2-256 hash of the recipient's public key)               |
| quantity         | BINARY           | Amount of Winstons sent to the recipient, if applicable                              |
| data             | BINARY           | Data uploaded with the transaction, if any                                           |
| data_size        | BINARY           | Size of the transaction data in bytes                                                |
| data_root        | VARCHAR          | Merkle root of the tree of data chunks                                               |
| signature        | VARCHAR          | Signature of the transaction                                                         |
| reward           | BINARY           | Fee paid for the transaction in Winstons                                             |

</details>

### `transaction_tags`

<details>
<summary>Click to expand Snowflake Table Schema for "transaction_tags"</summary>

| Column Name      | Data Type        | Description                                                                     |
| ---------------- | ---------------- | ------------------------------------------------------------------------------- |
| block_time       | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this transaction tag was created |
| block_height     | NUMBER(38,0)     | Height of the block in the Arweave chain containing this transaction tag        |
| block_date       | DATE             | Calendar date associated with the block containing this transaction tag         |
| block_indep_hash | VARCHAR          | Unique `indep_hash` of the block containing this transaction tag                |
| tx_id            | VARCHAR          | Unique identifier of the transaction associated with this tag                   |
| index            | VARCHAR          | Index of the tag in the transaction's tag list                                  |
| name             | VARCHAR          | Name of the tag                                                                 |
| value            | VARCHAR          | Value of the tag                                                                |