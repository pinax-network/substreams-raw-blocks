# `Bitcoin`: Snowflake Datashare

## Data Dictionary

### `blocks`

<details>
<summary>Click to expand Snowflake Table Schema for "blocks"</summary>

| Column Name         | Data Type        | Description                                                     |
| ------------------- | ---------------- | --------------------------------------------------------------- |
| time                | TIMESTAMP_NTZ(3) | Timestamp indicating when the block was created                 |
| height              | NUMBER(10,0)     | Height of the block in the Bitcoin chain                        |
| date                | DATE             | Calendar date associated with the block                         |
| hash                | VARCHAR          | Unique identifier (hash) of the block                           |
| bits                | VARCHAR          | Encoded compact form of the block's target threshold            |
| chainwork           | VARCHAR          | Cumulative proof-of-work for the chain up to this block         |
| difficulty          | DOUBLE           | Mining difficulty of the block                                  |
| merkle_root         | VARCHAR          | Merkle root hash of all transactions in the block               |
| transaction_count   | NUMBER(38,0)     | Total number of transactions in the block                       |
| nonce               | NUMBER(10,0)     | Nonce used to solve the proof-of-work for the block             |
| coinbase            | VARCHAR          | Coinbase transaction ID (first transaction in the block)        |
| previous_block_hash | VARCHAR          | Hash of the previous block, linking to the parent block         |
| version             | NUMBER(10,0)     | Version of the block as defined by the Bitcoin protocol         |
| weight              | NUMBER(10,0)     | Weight of the block, including witness data, in weight units    |
| size                | NUMBER(10,0)     | Size of the block in bytes                                      |
| stripped_size       | NUMBER(10,0)     | Size of the block without witness data in bytes                 |
| total_fees          | DOUBLE           | Total transaction fees included in the block (in Bitcoin)       |
| total_reward        | DOUBLE           | Total block reward (coinbase + fees) for the miner (in Bitcoin) |
| mint_reward         | DOUBLE           | New coins minted as a reward for mining the block (in Bitcoin)  |
| total_inputs        | NUMBER(10,0)     | Total number of transaction inputs in the block                 |
| total_outputs       | NUMBER(10,0)     | Total number of transaction outputs in the block                |


</details>


### `transactions`

<details>
<summary>Click to expand Snowflake Table Schema for "transactions"</summary>

| Column Name          | Data Type           | Description                                                                 |
| -------------------- | ------------------- | --------------------------------------------------------------------------- |
| block_time           | TIMESTAMP_NTZ(3)    | Timestamp indicating when the block containing this transaction was created |
| block_date           | DATE                | Calendar date associated with the block containing this transaction         |
| block_height         | NUMBER(10,0)        | Height of the block in the Bitcoin chain containing this transaction        |
| block_hash           | VARCHAR             | Unique identifier (hash) of the block containing this transaction           |
| index                | NUMBER(10,0)        | Index of the transaction within the block                                   |
| id                   | VARCHAR             | Unique identifier (hash) of the transaction                                 |
| lock_time            | NUMBER(10,0)        | Lock time for the transaction, specifying when it can be added to a block   |
| size                 | NUMBER(10,0)        | Size of the transaction in bytes                                            |
| virtual_size         | NUMBER(10,0)        | Virtual size of the transaction in bytes, considering SegWit discount       |
| coinbase             | VARCHAR             | Coinbase data for the transaction, if applicable                            |
| is_coinbase          | BOOLEAN             | Indicates whether the transaction is a coinbase transaction                 |
| version              | NUMBER(38,0)        | Version of the transaction as defined by the Bitcoin protocol               |
| input_count          | NUMBER(10,0)        | Total number of inputs in the transaction                                   |
| output_count         | NUMBER(10,0)        | Total number of outputs in the transaction                                  |
| input_tx_ids         | ARRAY<VARCHAR>      | List of transaction IDs for inputs referenced in this transaction           |
| input_output_indices | ARRAY<NUMBER(10,0)> | List of output indices for the inputs in this transaction                   |
| output_values        | ARRAY<DOUBLE>       | List of output values in Bitcoin for each output in this transaction        |
| hex                  | VARCHAR             | Raw hexadecimal representation of the transaction                           |

</details>

### `inputs`

<details>
<summary>Click to expand Snowflake Table Schema for "inputs"</summary>

| Column Name          | Data Type        | Description                                                           |
| -------------------- | ---------------- | --------------------------------------------------------------------- |
| block_time           | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this input was created |
| block_date           | DATE             | Calendar date associated with the block containing this input         |
| block_height         | NUMBER(10,0)     | Height of the block in the Bitcoin chain containing this input        |
| block_hash           | VARCHAR          | Unique identifier (hash) of the block containing this input           |
| tx_id                | VARCHAR          | Unique identifier (hash) of the transaction containing this input     |
| index                | NUMBER(10,0)     | Index of the input within the transaction                             |
| spent_block_height   | NUMBER(10,0)     | Height of the block where the output being spent was created          |
| spent_tx_id          | VARCHAR          | Transaction ID of the output being spent                              |
| spent_output_number  | NUMBER(10,0)     | Output index in the spending transaction                              |
| value                | DOUBLE           | Value of the input in Bitcoin                                         |
| address              | VARCHAR          | Address associated with this input                                    |
| type                 | VARCHAR          | Type of input                                                         |
| coinbase             | VARCHAR          | Coinbase data, if applicable                                          |
| is_coinbase          | BOOLEAN          | Indicates whether this input is part of a coinbase transaction        |
| script_asm           | VARCHAR          | (Optional) Assembly representation of the input script                |
| script_hex           | VARCHAR          | (Optional) Hexadecimal representation of the input script             |
| script_desc          | VARCHAR          | (Optional) Description of the input script, if available              |
| script_signature_asm | VARCHAR          | (Optional) Assembly representation of the script signature            |
| script_signature_hex | VARCHAR          | (Optional) Hexadecimal representation of the script signature         |
| sequence             | NUMBER(10,0)     | Sequence number for this input                                        |
| witness_data         | ARRAY<VARCHAR>   | List of witness data strings for this input, if applicable            |

</details>

### `outputs`

<details>
<summary>Click to expand Snowflake Table Schema for "ouputs"</summary>

| Column Name  | Data Type        | Description                                                            |
| ------------ | ---------------- | ---------------------------------------------------------------------- |
| block_time   | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this output was created |
| block_date   | DATE             | Calendar date associated with the block containing this output         |
| block_height | NUMBER(10,0)     | Height of the block in the Bitcoin chain containing this output        |
| block_hash   | VARCHAR          | Unique identifier (hash) of the block containing this output           |
| tx_id        | VARCHAR          | Unique identifier (hash) of the transaction containing this output     |
| index        | NUMBER(10,0)     | Index of the output within the transaction                             |
| value        | DOUBLE           | Value of the output in Bitcoin                                         |
| address      | VARCHAR          | Address associated with this output                                    |
| type         | VARCHAR          | Type of output (eg, P2PKH, P2SH)                                       |
| script_asm   | VARCHAR          | Assembly representation of the output script                           |
| script_hex   | VARCHAR          | Hexadecimal representation of the output script                        |

</details>






