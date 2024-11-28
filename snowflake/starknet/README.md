# `Starknet`: Snowflake Datashare

## Data Dictionary

### `blocks`

<details>
<summary>Click to expand Snowflake Table Schema for "blocks"</summary>

| Column Name              | Data Type        | Description                                                                      |
| ------------------------ | ---------------- | -------------------------------------------------------------------------------- |
| time                     | TIMESTAMP_NTZ(3) | Timestamp indicating when the block was created.                                 |
| number                   | NUMBER(38,0)     | Height of the block in the Starknet chain.                                       |
| date                     | DATE             | Calendar date associated with the block.                                         |
| hash                     | VARCHAR          | Unique identifier (hash) of the block.                                           |
| l1_da_mode               | VARCHAR          | Layer 1 data availability mode for the block (e.g., "validity", "availability"). |
| l1_data_gas_price_in_fri | BINARY           | Gas price for Layer 1 data in FRI (Field-Reduction Iteration).                   |
| l1_data_gas_price_in_wei | BINARY           | Gas price for Layer 1 data in Wei.                                               |
| l1_gas_price_in_fri      | BINARY           | Layer 1 gas price in FRI.                                                        |
| l1_gas_price_in_wei      | BINARY           | Layer 1 gas price in Wei.                                                        |
| starknet_version         | VARCHAR          | Version of the Starknet protocol used for the block.                             |
| tx_count                 | NUMBER(10,0)     | Total number of transactions in the block.                                       |
| new_root                 | VARCHAR          | New state root hash after processing the block.                                  |
| parent_hash              | VARCHAR          | Hash of the parent block linking this block to its predecessor.                  |
| sequencer_address        | VARCHAR          | Address of the sequencer that created this block.                                |
| state_diff               | VARCHAR          | State difference object representing changes in the state tree.                  |

</details>

### `transactions`

<details>
<summary>Click to expand Snowflake Table Schema for "transactions"</summary>

| Column Name                                          | Data Type        | Description                                                                  |
| ---------------------------------------------------- | ---------------- | ---------------------------------------------------------------------------- |
| block_time                                           | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this transaction was created. |
| block_date                                           | DATE             | Calendar date associated with the block containing this transaction.         |
| block_number                                         | NUMBER(38,0)     | Height of the block in the Starknet chain containing this transaction.       |
| block_hash                                           | VARCHAR          | Unique identifier (hash) of the block containing this transaction.           |
| block_l1_da_mode                                     | VARCHAR          | Layer 1 data availability mode of the block (e.g., "validity").              |
| block_l1_data_gas_price_in_fri                       | BINARY           | Layer 1 data gas price in FRI for the block.                                 |
| block_l1_data_gas_price_in_wei                       | BINARY           | Layer 1 data gas price in Wei for the block.                                 |
| block_l1_gas_price_in_fri                            | BINARY           | Layer 1 gas price in FRI for the block.                                      |
| block_l1_gas_price_in_wei                            | BINARY           | Layer 1 gas price in Wei for the block.                                      |
| block_starknet_version                               | VARCHAR          | Starknet version used in the block.                                          |
| fee_data_availability_mode                           | VARCHAR          | Data availability mode for transaction fees.                                 |
| hash                                                 | VARCHAR          | Unique identifier (hash) of the transaction.                                 |
| index                                                | NUMBER(10,0)     | Index of the transaction within the block.                                   |
| max_fee                                              | BINARY           | Maximum fee specified for the transaction.                                   |
| nonce                                                | BINARY           | Nonce for the transaction.                                                   |
| nonce_data_availability_mode                         | VARCHAR          | Data availability mode for the transaction nonce.                            |
| resource_bounds_l1_gas_max_amount                    | BINARY           | Maximum amount of Layer 1 gas allowed for the transaction.                   |
| resource_bounds_l1_gas_max_price_per_unit            | BINARY           | Maximum price per unit of Layer 1 gas for the transaction.                   |
| tip                                                  | BINARY           | Additional tip for the transaction fee.                                      |
| type                                                 | VARCHAR          | Type of transaction (e.g., "invoke").                                        |
| version                                              | VARCHAR          | Version of the transaction.                                                  |
| actual_fee_amount                                    | BINARY           | Actual fee paid for the transaction.                                         |
| actual_fee_unit                                      | VARCHAR          | Unit of the actual fee (e.g., "wei").                                        |
| execution_resources_bitwise_builtin_applications     | NUMBER(38,0)     | Count of bitwise operations in built-in applications for the transaction.    |
| execution_resources_data_availability_l1_gas         | NUMBER(38,0)     | Layer 1 gas used for data availability.                                      |
| execution_resources_data_availability_l1_data_gas    | NUMBER(38,0)     | Layer 1 data gas used for data availability.                                 |
| execution_resources_ec_op_builtin_applications       | NUMBER(38,0)     | Count of elliptic curve operations in built-in applications.                 |
| execution_resources_ecdsa_builtin_applications       | NUMBER(38,0)     | Count of ECDSA operations in built-in applications.                          |
| execution_resources_keccak_builtin_applications      | NUMBER(38,0)     | Count of Keccak operations in built-in applications.                         |
| execution_resources_memory_holes                     | NUMBER(38,0)     | Memory holes allocated during the transaction execution.                     |
| execution_resources_pedersen_builtin_applications    | NUMBER(38,0)     | Count of Pedersen operations in built-in applications.                       |
| execution_resources_poseidon_builtin_applications    | NUMBER(38,0)     | Count of Poseidon operations in built-in applications.                       |
| execution_resources_range_check_builtin_applications | NUMBER(38,0)     | Count of range checks in built-in applications.                              |
| execution_resources_segment_arena_builtin            | NUMBER(38,0)     | Resources used in segment arena built-ins.                                   |
| execution_resources_steps                            | NUMBER(38,0)     | Number of steps taken during transaction execution.                          |
| execution_status                                     | VARCHAR          | Execution status of the transaction (e.g., "SUCCEEDED").                     |
| finality_status                                      | VARCHAR          | Finality status of the transaction.                                          |
| receipt_type                                         | VARCHAR          | Type of transaction receipt.                                                 |
| block_new_root                                       | VARCHAR          | New state root after applying the transaction.                               |
| block_parent_hash                                    | VARCHAR          | Hash of the parent block in the chain.                                       |
| block_sequencer_address                              | VARCHAR          | Address of the sequencer that processed the block.                           |
| calldata                                             | ARRAY<VARCHAR>   | List of calldata elements for the transaction.                               |
| class_hash                                           | VARCHAR          | Hash of the class used in the transaction.                                   |
| compiled_class_hash                                  | VARCHAR          | Hash of the compiled class used in the transaction.                          |
| constructor_calldata                                 | ARRAY<VARCHAR>   | Calldata for the contract constructor, if applicable.                        |
| contract_address                                     | VARCHAR          | Address of the contract involved in the transaction.                         |
| contract_address_salt                                | VARCHAR          | Salt used for generating the contract address.                               |
| entry_point_selector                                 | VARCHAR          | Selector for the entry point invoked in the transaction.                     |
| sender_address                                       | VARCHAR          | Address of the sender of the transaction.                                    |
| signature                                            | ARRAY<VARCHAR>   | List of signature elements for the transaction.                              |
| message_hash                                         | VARCHAR          | Hash of the message associated with the transaction.                         |
| revert_reason                                        | VARCHAR          | Reason for transaction revert, if applicable.                                |

</details>


### `messages_sent`

<details>
<summary>Click to expand Snowflake Table Schema for "transactions"</summary>

| Column Name                    | Data Type        | Description                                                              |
| ------------------------------ | ---------------- | ------------------------------------------------------------------------ |
| block_time                     | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this message was created. |
| block_number                   | NUMBER(38,0)     | Height of the block in the Starknet chain containing this message.       |
| block_date                     | DATE             | Calendar date associated with the block containing this message.         |
| block_hash                     | VARCHAR          | Unique identifier (hash) of the block containing this message.           |
| block_l1_da_mode               | VARCHAR          | Layer 1 data availability mode for the block (e.g., "validity").         |
| block_l1_data_gas_price_in_fri | BINARY           | Gas price for Layer 1 data in FRI for the block.                         |
| block_l1_data_gas_price_in_wei | BINARY           | Gas price for Layer 1 data in Wei for the block.                         |
| block_l1_gas_price_in_fri      | BINARY           | Layer 1 gas price in FRI for the block.                                  |
| block_l1_gas_price_in_wei      | BINARY           | Layer 1 gas price in Wei for the block.                                  |
| block_starknet_version         | VARCHAR          | Starknet version used in the block.                                      |
| tx_index                       | NUMBER(10,0)     | Index of the transaction within the block.                               |
| tx_type                        | VARCHAR          | Type of transaction sending the message (e.g., "invoke").                |
| from_address                   | VARCHAR          | Address of the sender of the message.                                    |
| payload                        | ARRAY<VARCHAR>   | List of payload elements in the message.                                 |
| to_address                     | VARCHAR          | Address of the recipient of the message.                                 |


</details>

### `events`

<details>
<summary>Click to expand Snowflake Table Schema for "events"</summary>

| Column Name                    | Data Type        | Description                                                              |
| ------------------------------ | ---------------- | ------------------------------------------------------------------------ |
| block_time                     | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this event was created.   |
| block_number                   | NUMBER(38,0)     | Height of the block in the Starknet chain containing this event.         |
| block_date                     | DATE             | Calendar date associated with the block containing this event.           |
| block_hash                     | VARCHAR          | Unique identifier (hash) of the block containing this event.             |
| block_new_root                 | VARCHAR          | New state root hash after processing the block containing this event.    |
| block_parent_hash              | VARCHAR          | Hash of the parent block in the chain.                                   |
| block_sequencer_address        | VARCHAR          | Address of the sequencer that processed the block containing this event. |
| block_starknet_version         | VARCHAR          | Starknet version used in the block.                                      |
| block_l1_da_mode               | VARCHAR          | Layer 1 data availability mode for the block (e.g., "validity").         |
| block_l1_data_gas_price_in_fri | BINARY           | Gas price for Layer 1 data in FRI for the block.                         |
| block_l1_data_gas_price_in_wei | BINARY           | Gas price for Layer 1 data in Wei for the block.                         |
| block_l1_gas_price_in_fri      | BINARY           | Layer 1 gas price in FRI for the block.                                  |
| block_l1_gas_price_in_wei      | BINARY           | Layer 1 gas price in Wei for the block.                                  |
| event_index                    | NUMBER(10,0)     | Index of the event within the transaction.                               |
| tx_hash                        | VARCHAR          | Unique identifier (hash) of the transaction containing this event.       |
| data                           | ARRAY<VARCHAR>   | Data associated with the event.                                          |
| keys                           | ARRAY<VARCHAR>   | List of keys associated with the event.                                  |
| from_address                   | VARCHAR          | Address of the contract that emitted the event.                          |
| class_hash                     | VARCHAR          | Class hash of the contract emitting the event.                           |

</details>


### `calls`

**NOT YET IMPLEMENTED IN SUBSTREAM**

<details>
<summary>Click to expand Snowflake Table Schema for "calls"</summary>

| Column Name                                          | Data Type           | Description                                                           |
| ---------------------------------------------------- | ------------------- | --------------------------------------------------------------------- |
| block_time                                           | TIMESTAMP_NTZ(3)    | Timestamp indicating when the block containing this call was created. |
| block_number                                         | NUMBER(38,0)        | Height of the block in the Starknet chain containing this call.       |
| block_date                                           | DATE                | Calendar date associated with the block containing this call.         |
| block_hash                                           | VARCHAR             | Unique identifier (hash) of the block containing this call.           |
| block_l1_da_mode                                     | VARCHAR             | Layer 1 data availability mode for the block.                         |
| block_l1_data_gas_price_in_fri                       | BINARY              | Gas price for Layer 1 data in FRI for the block.                      |
| block_l1_data_gas_price_in_wei                       | BINARY              | Gas price for Layer 1 data in Wei for the block.                      |
| block_l1_gas_price_in_fri                            | BINARY              | Layer 1 gas price in FRI for the block.                               |
| block_l1_gas_price_in_wei                            | BINARY              | Layer 1 gas price in Wei for the block.                               |
| block_starknet_version                               | VARCHAR             | Starknet version used in the block.                                   |
| tx_index                                             | NUMBER(10,0)        | Index of the transaction within the block.                            |
| tx_type                                              | VARCHAR             | Type of the transaction associated with the call (e.g., "invoke").    |
| call_type                                            | VARCHAR             | Type of the call (e.g., "delegatecall", "call").                      |
| context                                              | VARCHAR             | Context of the call (e.g., specific contract state).                  |
| entry_point_type                                     | VARCHAR             | Type of the entry point invoked (e.g., "external").                   |
| execution_resources_bitwise_builtin_applications     | NUMBER(10,0)        | Count of bitwise operations in built-in applications for this call.   |
| execution_resources_data_availability_l1_gas         | NUMBER(38,0)        | Layer 1 gas used for data availability in this call.                  |
| execution_resources_data_availability_l1_data_gas    | NUMBER(38,0)        | Layer 1 data gas used for data availability in this call.             |
| execution_resources_ec_op_builtin_applications       | NUMBER(10,0)        | Count of elliptic curve operations in built-in applications.          |
| execution_resources_ecdsa_builtin_applications       | NUMBER(10,0)        | Count of ECDSA operations in built-in applications.                   |
| execution_resources_keccak_builtin_applications      | NUMBER(10,0)        | Count of Keccak operations in built-in applications.                  |
| execution_resources_memory_holes                     | NUMBER(10,0)        | Number of memory holes allocated during the call execution.           |
| execution_resources_pedersen_builtin_applications    | NUMBER(10,0)        | Count of Pedersen operations in built-in applications.                |
| execution_resources_poseidon_builtin_applications    | NUMBER(10,0)        | Count of Poseidon operations in built-in applications.                |
| execution_resources_range_check_builtin_applications | NUMBER(10,0)        | Count of range checks in built-in applications.                       |
| execution_resources_segment_arena_builtin            | NUMBER(10,0)        | Resources used in segment arena built-ins during the call.            |
| execution_resources_steps                            | NUMBER(10,0)        | Number of steps taken during the call execution.                      |
| num_subcalls                                         | NUMBER(10,0)        | Number of subcalls within this call.                                  |
| block_new_root                                       | VARCHAR             | New state root after applying the call.                               |
| block_parent_hash                                    | VARCHAR             | Hash of the parent block in the chain.                                |
| block_sequencer_address                              | VARCHAR             | Address of the sequencer processing the block containing this call.   |
| transaction_hash                                     | VARCHAR             | Hash of the transaction containing this call.                         |
| callstack_index                                      | ARRAY<NUMBER(10,0)> | List of indexes in the call stack for this call.                      |
| calldata                                             | ARRAY<VARCHAR>      | List of calldata elements for the call.                               |
| caller_address                                       | VARCHAR             | Address of the caller of this call.                                   |
| class_hash                                           | VARCHAR             | Hash of the class used in the call.                                   |
| contract_address                                     | VARCHAR             | Address of the contract being invoked in the call.                    |
| entry_point_selector                                 | VARCHAR             | Selector of the entry point invoked in the call.                      |
| result                                               | VARCHAR             | Result of the call execution.                                         |
| revert_reason                                        | VARCHAR             | Reason for the call's revert, if applicable.                          |
| state_diff                                           | VARCHAR             | State difference object representing changes caused by the call.      |

</details>