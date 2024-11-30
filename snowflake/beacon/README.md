# `Beacon`: Snowflake Datashare

## Data Dictionary

### `blocks`

<details>
<summary>Click to expand Snowflake Table Schema for "blocks"</summary>

| Column Name    | Data Type        | Description                                                               |
| -------------- | ---------------- | ------------------------------------------------------------------------- |
| time           | TIMESTAMP_NTZ(3) | Timestamp indicating when this block was created                          |
| number         | NUMBER(38,0)     | Sequential number of the block in the blockchain                          |
| date           | DATE             | Calendar date associated with the block                                   |
| hash           | VARCHAR          | Unique identifier (hash) of the block                                     |
| version        | INTEGER          | Version of the block as defined by the consensus specification            |
| spec           | VARCHAR          | Specification or protocol version associated with this block              |
| slot           | NUMBER(38,0)     | Slot number of the block, representing its position in the beacon chain   |
| parent_slot    | NUMBER(38,0)     | Slot number of the parent block directly preceding this block             |
| root           | VARCHAR          | Root hash of the block, representing its state in the chain               |
| parent_root    | VARCHAR          | Root hash of the parent block, linking this block to its predecessor      |
| state_root     | VARCHAR          | Root hash of the state after processing this block                        |
| proposer_index | NUMBER(38,0)     | Index of the validator that proposed this block                           |
| body_root      | VARCHAR          | Root hash of the block body, representing the transactions and other data |
| signature      | VARCHAR          | Signature of the block, used for validation by the proposer               |


</details>

### `blobs`

<details>
<summary>Click to expand Snowflake Table Schema for "blobs"</summary>

| Column Name                    | Data Type        | Description                                                           |
| ------------------------------ | ---------------- | --------------------------------------------------------------------- |
| block_time                     | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this blob was created  |
| block_number                   | NUMBER(38,0)     | Sequential number of the block in the blockchain containing this blob |
| block_date                     | DATE             | Calendar date associated with the block containing this blob          |
| block_hash                     | VARCHAR          | Unique identifier (hash) of the block containing this blob            |
| index                          | NUMBER(38,0)     | Index of the blob within the block                                    |
| blob                           | VARCHAR          | Encoded representation of the blob's data                             |
| kzg_commitment                 | VARCHAR          | KZG commitment for the blob, ensuring data integrity                  |
| kzg_proof                      | VARCHAR          | KZG proof for the blob, used to verify the commitment                 |
| kzg_commitment_inclusion_proof | ARRAY<VARCHAR>   | List of proofs for inclusion of the KZG commitment in the block       |

</details>

### `deposits`

<details>
<summary>Click to expand Snowflake Table Schema for "deposits"</summary>

| Column Name            | Data Type        | Description                                                                 |
| ---------------------- | ---------------- | --------------------------------------------------------------------------- |
| block_time             | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this deposit was created     |
| block_number           | NUMBER(38,0)     | Sequential number of the block in the blockchain containing this deposit    |
| block_date             | DATE             | Calendar date associated with the block containing this deposit             |
| block_hash             | VARCHAR          | Unique identifier (hash) of the block containing this deposit               |
| index                  | NUMBER(38,0)     | Index of the deposit within the block                                       |
| proof                  | ARRAY<VARCHAR>   | List of Merkle proofs for the deposit, verifying its inclusion in the block |
| pubkey                 | VARCHAR          | Public key associated with the validator making the deposit                 |
| withdrawal_credentials | VARCHAR          | Withdrawal credentials associated with the deposit                          |
| signature              | VARCHAR          | Signature of the deposit, used for verification                             |
| gwei                   | NUMBER(38,0)     | Amount of the deposit in gwei                                               |

</details>

### `withdrawals`

<details>
<summary>Click to expand Snowflake Table Schema for "withdrawals"</summary>

| Column Name      | Data Type        | Description                                                                 |
| ---------------- | ---------------- | --------------------------------------------------------------------------- |
| block_time       | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this withdrawal was created  |
| block_number     | NUMBER(38,0)     | Sequential number of the block in the blockchain containing this withdrawal |
| block_date       | DATE             | Calendar date associated with the block containing this withdrawal          |
| block_hash       | VARCHAR          | Unique identifier (hash) of the block containing this withdrawal            |
| withdrawal_index | NUMBER(38,0)     | Index of the withdrawal within the block                                    |
| validator_index  | NUMBER(38,0)     | Index of the validator associated with this withdrawal                      |
| address          | VARCHAR          | Address where the withdrawal is sent                                        |
| gwei             | NUMBER(38,0)     | Amount withdrawn in gwei (smallest unit of Ether)                           |

</details>

### `attestations`

<details>
<summary>Click to expand Snowflake Table Schema for "attestations"</summary>

| Column Name       | Data Type        | Description                                                                   |
| ----------------- | ---------------- | ----------------------------------------------------------------------------- |
| block_time        | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this attestation was created   |
| block_number      | NUMBER(38,0)     | Sequential number of the block in the blockchain containing this attestation  |
| block_date        | DATE             | Calendar date associated with the block containing this attestation           |
| block_hash        | VARCHAR          | Unique identifier (hash) of the block containing this attestation             |
| index             | NUMBER(38,0)     | Index of the attestation within the block                                     |
| aggregation_bits  | VARCHAR          | Bitfield indicating participation of committee members in the attestation     |
| slot              | NUMBER(38,0)     | Slot number of the attestation, representing its position in the beacon chain |
| committee_index   | NUMBER(38,0)     | Index of the committee responsible for this attestation                       |
| beacon_block_root | VARCHAR          | Root hash of the beacon block being attested                                  |
| source_epoch      | NUMBER(38,0)     | Starting epoch of the chain state referenced by the attestation               |
| source_root       | VARCHAR          | Root hash of the chain state at the source epoch                              |
| target_epoch      | NUMBER(38,0)     | Target epoch for the chain state referenced by the attestation                |
| target_root       | VARCHAR          | Root hash of the chain state at the target epoch                              |
| signature         | VARCHAR          | Signature of the attestation, used for verification                           |

</details>

### `attester_slashings`

<details>
<summary>Click to expand Snowflake Table Schema for "attester_slashings"</summary>

| Column Name       | Data Type           | Description                                                                        |
| ----------------- | ------------------- | ---------------------------------------------------------------------------------- |
| block_time        | TIMESTAMP_NTZ(3)    | Timestamp indicating when the block containing this attester slashing was created  |
| block_number      | NUMBER(38,0)        | Sequential number of the block in the blockchain containing this attester slashing |
| block_date        | DATE                | Calendar date associated with the block containing this attester slashing          |
| block_hash        | VARCHAR             | Unique identifier (hash) of the block containing this attester slashing            |
| index             | NUMBER(38,0)        | Index of the attester slashing within the block                                    |
| attesting_indices | ARRAY<NUMBER(38,0)> | List of validator indices involved in the slashing                                 |
| slot              | NUMBER(38,0)        | Slot number of the attestation associated with the slashing                        |
| committee_index   | NUMBER(38,0)        | Index of the committee responsible for the attestation                             |
| beacon_block_root | VARCHAR             | Root hash of the beacon block being attested                                       |
| source_epoch      | NUMBER(38,0)        | Starting epoch of the chain state referenced by the attestation                    |
| source_root       | VARCHAR             | Root hash of the chain state at the source epoch                                   |
| target_epoch      | NUMBER(38,0)        | Target epoch for the chain state referenced by the attestation                     |
| target_root       | VARCHAR             | Root hash of the chain state at the target epoch                                   |
| signature         | VARCHAR             | Signature of the attestation, used for verification                                |

</details>

### `bls_to_execution_changes`

<details>
<summary>Click to expand Snowflake Table Schema for "bls_to_execution_changes"</summary>

| Column Name          | Data Type        | Description                                                             |
| -------------------- | ---------------- | ----------------------------------------------------------------------- |
| block_time           | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this change was created  |
| block_number         | NUMBER(38,0)     | Sequential number of the block in the blockchain containing this change |
| block_date           | DATE             | Calendar date associated with the block containing this change          |
| block_hash           | VARCHAR          | Unique identifier (hash) of the block containing this change            |
| index                | NUMBER(38,0)     | Index of this change within the block                                   |
| validator_index      | NUMBER(38,0)     | Index of the validator associated with this change                      |
| from_bls_pubkey      | VARCHAR          | BLS public key being changed                                            |
| to_execution_address | VARCHAR          | Ethereum execution layer address receiving the delegation               |
| signature            | VARCHAR          | Signature of the change, used for verification                          |

</details>


### `proposer_slashings`

<details>
<summary>Click to expand Snowflake Table Schema for "proposer_slashings"</summary>

| Column Name    | Data Type        | Description                                                               |
| -------------- | ---------------- | ------------------------------------------------------------------------- |
| block_time     | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this slashing was created  |
| block_number   | NUMBER(38,0)     | Sequential number of the block in the blockchain containing this slashing |
| block_date     | DATE             | Calendar date associated with the block containing this slashing          |
| block_hash     | VARCHAR          | Unique identifier (hash) of the block containing this slashing            |
| index          | NUMBER(38,0)     | Index of the slashing within the block                                    |
| slot           | NUMBER(38,0)     | Slot number of the header involved in the slashing                        |
| proposer_index | NUMBER(38,0)     | Index of the proposer involved in the slashing                            |
| parent_root    | VARCHAR          | Root hash of the parent block for the signed header                       |
| state_root     | VARCHAR          | Root hash of the chain state for the signed header                        |
| body_root      | VARCHAR          | Root hash of the block body for the signed header                         |
| signature      | VARCHAR          | Signature of the header, used for verification                            |

</details>


### `voluntary_exits`

<details>
<summary>Click to expand Snowflake Table Schema for "voluntary_exits"</summary>

| Column Name     | Data Type        | Description                                                                     |
| --------------- | ---------------- | ------------------------------------------------------------------------------- |
| block_time      | TIMESTAMP_NTZ(3) | Timestamp indicating when the block containing this voluntary exit was created  |
| block_number    | NUMBER(38,0)     | Sequential number of the block in the blockchain containing this voluntary exit |
| block_date      | DATE             | Calendar date associated with the block containing this voluntary exit          |
| block_hash      | VARCHAR          | Unique identifier (hash) of the block containing this voluntary exit            |
| index           | NUMBER(38,0)     | Index of the voluntary exit within the block                                    |
| epoch           | NUMBER(38,0)     | Epoch during which the validator initiated the voluntary exit                   |
| validator_index | NUMBER(38,0)     | Index of the validator requesting the voluntary exit                            |
| signature       | VARCHAR          | Signature of the voluntary exit request, used for verification                  |

</details>










