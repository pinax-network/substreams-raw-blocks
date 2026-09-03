# `EVM` Raw Blockchain Data

> Ethereum, Base, BNB, Arbitrum One, Polygon, Avalanche, Optimism... any chain served as
> [`sf.ethereum.type.v2.Block`](https://buf.build/streamingfast/firehose-ethereum/docs/main:sf.ethereum.type.v2).

Output is shaped for [`substreams sink postgres`](https://docs.substreams.dev/how-to-guides/sinks/sql/relational-mappings)
(Relational Mappings Mode): every repeated field of `pinax.evm.v2.Events` is a table, no `schema.sql` needed.
Table and column naming follows [pinax-network/firehose-parquet](https://github.com/pinax-network/firehose-parquet/tree/main/blocks/src/evm),
extended with every additional field the Firehose block offers.

## Quick start

```bash
export SUBSTREAMS_API_KEY=...          # Pinax API key
export DSN="postgres://user:pass@localhost:5432/raw_blocks?sslmode=disable"

make build
make sink NETWORK=eth                   # every table
make sink NETWORK=base MODULE=map_events_no_storage_changes
make sink NETWORK=bsc PARAMS="exclude=gas_changes,keccak_preimages,system_gas_changes"
```

Binary columns (hashes, addresses, calldata...) are `BYTEA` by default; add `--bytes-encoding 0xhex`
to the sink command (`ARGS="--bytes-encoding 0xhex"`) to store them as `0x...` text instead.

## Modules

| Module | Output | Purpose |
|---|---|---|
| `map_events` | `pinax.evm.v2.Events` | Every table. `params` selects tables: `exclude=a,b` or `include=a,b` |
| `map_events_no_storage_changes` | `pinax.evm.v2.Events` | `map_events` with `exclude=storage_changes,system_storage_changes` baked in |
| `map_block` | `sf.ethereum.type.v2.Block` | Firehose block passthrough, for `substreams estimate` |
| `map_block_no_storage_changes` | `sf.ethereum.type.v2.Block` | Same block with every `storage_changes` field removed |
| `map_block_sizes` | `pinax.evm.v2.BlockSizes` | One row per block: encoded bytes of the block and of each data family |

## Tables

Every table starts with the canonical identity columns `block_num, block_id, parent_num, parent_id, timestamp, date`
(the sink adds its own `_block_number_`, `_block_timestamp_` and a `_blocks_` table).

| Table | Detail level | Source |
|---|---|---|
| `blocks` | BASE | header, `num_transactions`, `detail_level`, blob/withdrawal/requests roots |
| `uncles` | BASE | ommer headers |
| `transactions` | BASE | all transactions (including failed/reverted), signature, blob fields, receipt roots |
| `logs` | BASE | receipt logs (`topic0..3`, `data`) |
| `access_lists` | BASE | EIP-2930 access list entries |
| `set_code_authorizations` | BASE | EIP-7702 authorizations |
| `calls` | EXTENDED | call tree (`input`, `output`, status flags, `failure_reason`) |
| `balance_changes`, `code_changes`, `storage_changes`, `nonce_changes`, `gas_changes`, `account_creations` | EXTENDED | state changes recorded in transaction calls |
| `keccak_preimages` | EXTENDED | keccak preimages recorded in transaction calls |
| `system_calls` | EXTENDED | system calls (Cancun+), executed outside any transaction |
| `system_balance_changes`, `system_code_changes`, `system_storage_changes`, `system_nonce_changes`, `system_gas_changes`, `system_account_creations`, `system_keccak_preimages` | EXTENDED | block-level changes (rewards, withdrawals, genesis, system calls) |

Column types: hashes/addresses/payloads are `bytes`, uint256 values are `NUMERIC(78,0)`,
enumerations are the Firehose names without prefix (`EXTENDED`, `DYNAMIC_FEE`, `SUCCEEDED`, `DELEGATE`, `REWARD_TRANSACTION_FEE`...).

## Estimating egress (with vs. without storage changes)

```bash
# Full-history estimate, sampling 1% of the range (every sampled block is streamed back)
make estimate NETWORK=eth START=0 STOP=0 SAMPLE=1

# Exact per-block breakdown over a sample range
make sizes NETWORK=base START=24450000 STOP=+100
```
