# `Cosmos` Raw Blockchain Data

> CosmosHub, Injective, Osmosis... any chain served as
> [`sf.cosmos.type.v2.Block`](https://buf.build/streamingfast/firehose-cosmos/docs/main:sf.cosmos.type.v2).

Output is shaped for [`substreams sink postgres`](https://docs.substreams.dev/how-to-guides/sinks/sql/relational-mappings):
every repeated field of `pinax.cosmos.v2.Events` is a table. Naming follows
[pinax-network/firehose-parquet](https://github.com/pinax-network/firehose-parquet/tree/main/blocks/src/cosmos),
extended with every additional field the Firehose block offers.

```bash
export SUBSTREAMS_API_KEY=...
export DSN="postgres://user:pass@localhost:5432/raw_blocks_cosmos?sslmode=disable"
make build && make sink-setup && make sink NETWORK=cosmoshub   # or injective, osmosis...
```

## Tables

Every table starts with `block_num, block_id, parent_num, parent_id, timestamp, date`.
Hashes and addresses are `bytes` (`--bytes-encoding hex` for text); the transaction hash is `sha256(raw_tx)`.

| Table | Source |
|---|---|
| `blocks` | header hashes, chain id, proposer, consensus version, counters |
| `transactions` | tx hash (PK), result code/gas/log/info/codespace, `success`, result `data`, `raw_tx` bytes |
| `events` | one row per event attribute, block-level (`source = block`) and per transaction (`source = transaction`) |
| `messages` | `cosmos.tx.v1beta1.Tx.body.messages` as `type_url` + raw `value` |
| `misbehaviors`, `validator_updates`, `consensus_param_updates` | consensus-level updates |
