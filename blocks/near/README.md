# `Near` Raw Blockchain Data

> NEAR Protocol, served as [`sf.near.type.v1.Block`](https://buf.build/streamingfast/firehose-near/docs/main:sf.near.type.v1).

Output is shaped for [`substreams sink postgres`](https://docs.substreams.dev/how-to-guides/sinks/sql/relational-mappings):
every repeated field of `pinax.near.v2.Events` is a table. Naming follows
[pinax-network/firehose-parquet](https://github.com/pinax-network/firehose-parquet/tree/main/blocks/src/near),
extended with every additional field the Firehose block offers and a flattened `actions` table.

```bash
export SUBSTREAMS_API_KEY=...
export DSN="postgres://user:pass@localhost:5432/raw_blocks_near?sslmode=disable"
make build && make sink-setup ARGS="--bytes-encoding base58" && make sink NETWORK=near ARGS="--bytes-encoding base58"
```

## Tables

Every table starts with `block_num` (= height), `block_id` (= block hash), `parent_num` (= prev height), `parent_id`, `timestamp`, `date`.
Hashes and keys are `bytes`; yoctoNEAR amounts are `NUMERIC(78,0)`.

| Table | Source |
|---|---|
| `blocks` | full block header (roots, epoch ids, gas price, total supply, finality pointers, chunk mask), counters |
| `chunks` | chunk headers of the shards that carry a chunk (missing chunks are skipped) |
| `transactions` | signed transactions with their execution outcome (`status`, `gas_burnt`, `tokens_burnt`, logs, receipt ids, decoded failure) |
| `receipts` | executed receipts (`kind = action | data`) with signer, gas price, data payload and outcome |
| `actions` | one row per action of transactions (`source = transaction`) and action receipts (`source = receipt`), typed columns per action kind |
| `state_changes` | account / access key / data / contract code changes with their cause and referenced tx or receipt hash |
