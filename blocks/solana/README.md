# `Solana` Raw Blockchain Data

> Solana, served as [`sf.solana.type.v1.Block`](https://buf.build/streamingfast/firehose-solana/docs/main:sf.solana.type.v1).

Output is shaped for [`substreams sink postgres`](https://docs.substreams.dev/how-to-guides/sinks/sql/relational-mappings)
(Relational Mappings Mode): every repeated field of `pinax.solana.v2.Events` is a table.
Naming follows [pinax-network/firehose-parquet](https://github.com/pinax-network/firehose-parquet/tree/main/blocks/src/solana),
extended with resolved addresses, decoded transaction errors and an `account_activity` table.

## Quick start

```bash
export SUBSTREAMS_API_KEY=...
export DSN="postgres://user:pass@localhost:5432/raw_blocks_solana?sslmode=disable"

make build
make sink-setup ARGS="--bytes-encoding base58"   # addresses/signatures as base58 text
make sink ARGS="--bytes-encoding base58"         # without vote transactions
make sink MODULE=map_events_with_votes ARGS="--bytes-encoding base58"
```

Without `--bytes-encoding base58` addresses, hashes and signatures are stored as `BYTEA`.

## Modules

| Module | Input | Output |
|---|---|---|
| `map_events` | `solana-common:blocks_without_votes` | every table except `vote_transactions` |
| `map_events_with_votes` | full `sf.solana.type.v1.Block` | same, plus vote transactions in `vote_transactions` (no messages/instructions for votes) |

## Tables

Every table starts with `block_num` (= slot), `block_id` (= blockhash), `parent_num` (= parent slot), `parent_id`, `timestamp`, `date`.

| Table | Source |
|---|---|
| `blocks` | slot, height, hashes, block time, transaction/reward counters (incl. vote and failed counts) |
| `transactions` / `vote_transactions` | signature (PK), fee, success, raw `err` + decoded `error`, compute/cost units, log messages, pre/post balances, return data, all signatures, signer |
| `messages` | header counts, recent blockhash, versioned flag, static account keys, loaded writable/readonly addresses |
| `instructions` | top-level then inner instructions (`is_inner`, `inner_index`, `stack_height`), raw indexes + data, resolved `program_id` and `account_addresses` |
| `rewards` | block rewards (`source = block`) and per-transaction rewards (`source = transaction`) |
| `token_balances` | pre/post token balances with `amount` as `NUMERIC`, resolved `account` |
| `account_lookups` | address table lookups of versioned transactions |
| `account_activity` | one row per account per transaction: signed/writable flags, SOL balance change, token pre/post amounts |
