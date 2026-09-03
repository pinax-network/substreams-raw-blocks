# `Antelope` Raw Blockchain Data

> EOS, WAX, Telos, Ultra... any chain served as
> [`sf.antelope.type.v1.Block`](https://buf.build/pinax/firehose-antelope/docs/main:sf.antelope.type.v1).

Output is shaped for [`substreams sink postgres`](https://docs.substreams.dev/how-to-guides/sinks/sql/relational-mappings)
(Relational Mappings Mode): every repeated field of `pinax.antelope.v2.Events` is a table.
Naming follows [pinax-network/firehose-parquet](https://github.com/pinax-network/firehose-parquet/tree/main/blocks/src/antelope),
extended with every additional field the Firehose block offers.

## Quick start

```bash
export SUBSTREAMS_API_KEY=...
export DSN="postgres://user:pass@localhost:5432/raw_blocks_antelope?sslmode=disable"

make build
make sink-setup                 # tables + widen string columns to TEXT
make sink NETWORK=eos           # or wax, telos, ultra...
```

## Tables

Every table starts with `block_num, block_id, parent_num, parent_id, timestamp, date`.
Antelope identifiers stay hex `string`s (as in Firehose); raw payloads are `bytes`.

| Table | Source |
|---|---|
| `blocks` | header, producer, merkle roots, DPoS/Savanna finality numbers, counters |
| `transactions` | every transaction trace (all statuses), receipt usage, exception (JSON), creation tree, signatures |
| `actions` | action traces with receipt, authorization (`actor@permission,...`), json/raw data, console, return values, RAM deltas (JSON) |
| `db_ops` | table row changes (`INSERT`/`UPDATE`/`REMOVE`), raw and JSON data |
| `feature_ops`, `perm_ops`, `table_ops`, `ram_ops`, `ram_correction_ops`, `dtrx_ops` | the other per-transaction operation lists |

Enumerations use the Firehose names without prefix (`EXECUTED`, `SOFTFAIL`, `INSERT`, `PRIMARY_INDEX_ADD`, `TABLE_ROW`...).
