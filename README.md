# Substreams Raw Blocks

> Raw blockchain data as Substreams, shaped for [`substreams sink postgres`](https://docs.substreams.dev/how-to-guides/sinks/sql/relational-mappings)
> (one table per data type, no `schema.sql` to write). Table layouts follow
> [pinax-network/firehose-parquet](https://github.com/pinax-network/firehose-parquet).

## `BlockType` Support

| Status   | BlockType | Chains |
|----------|-----------|--------|
| ✅ | [`EVM`](/blocks/evm)             | Ethereum, Base, Arbitrum One, Polygon, BNB, Avalanche... |
| ✅ | [`Antelope`](/blocks/antelope)   | WAX, EOS, Ultra, Telos... |
| ✅ | [`Solana`](/blocks/solana)       | Solana |
| ✅ | [`Cosmos`](/blocks/cosmos)       | CosmosHub, Injective, Osmosis...  |
| ✅ | [`Beacon`](/blocks/beacon)       | Ethereum 2.0 Beacon Chain |
| ✅ | [`Bitcoin`](/blocks/bitcoin)     | Bitcoin, Litecoin, Dogecoin... |
| ✅ | [`Starknet`](/blocks/starknet)   | Starknet |
| ✅ | [`Arweave`](/blocks/arweave)     | Arweave |
| ✅ | [`Near`](/blocks/near)           | NEAR Protocol |

> ✅ Supported, 🚧 In Progress, ⌛ Planned
>
> All packages ship the v2 schema: every repeated field of `pinax.<chain>.v2.Events` is a Postgres table,
> each table starts with the canonical identity columns `block_num, block_id, parent_num, parent_id, timestamp, date`,
> hashes/addresses/payloads are `bytes` (pick the text form with `--bytes-encoding hex|0xhex|base58|base64`),
> big numbers are `NUMERIC(78,0)`, and enumerations use the Firehose names. Table layouts follow
> [pinax-network/firehose-parquet](https://github.com/pinax-network/firehose-parquet) where it models the chain
> (EVM, Antelope, Solana, Bitcoin, Beacon, Cosmos, Near) and are extended with every extra field the Firehose block offers.

## Quick start (Postgres)

```bash
export SUBSTREAMS_API_KEY=...   # Pinax API key
export DSN="postgres://user:pass@localhost:5432/raw_blocks?sslmode=disable"

cd blocks/evm
make build                                   # cargo build + substreams pack
make sink-setup                              # create tables, widen string columns to TEXT
make sink NETWORK=eth                        # backfill + live
make sink NETWORK=base MODULE=map_events_no_storage_changes
```

`sink-setup` applies [`sql/varchar_to_text.sql`](sql/varchar_to_text.sql): the sink maps protobuf `string`
to `VARCHAR(255)`, which raw block data (JSON payloads, console output, revert reasons) exceeds.
