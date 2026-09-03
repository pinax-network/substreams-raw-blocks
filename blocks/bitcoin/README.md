# `Bitcoin` Raw Blockchain Data

> Bitcoin, Litecoin, Dogecoin... any chain served as
> [`sf.bitcoin.type.v1.Block`](https://buf.build/streamingfast/firehose-bitcoin/docs/main:sf.bitcoin.type.v1).

Output is shaped for [`substreams sink postgres`](https://docs.substreams.dev/how-to-guides/sinks/sql/relational-mappings):
every repeated field of `pinax.bitcoin.v2.Events` is a table. Naming follows
[pinax-network/firehose-parquet](https://github.com/pinax-network/firehose-parquet/tree/main/blocks/src/bitcoin),
extended with every additional field the Firehose block offers.

```bash
export SUBSTREAMS_API_KEY=...
export DSN="postgres://user:pass@localhost:5432/raw_blocks_bitcoin?sslmode=disable"
make build && make sink-setup && make sink NETWORK=bitcoin
```

## Tables

Every table starts with `block_num, block_id, parent_num, parent_id, timestamp, date`.
Bitcoin hashes stay hex `string`s (as in Firehose).

| Table | Source |
|---|---|
| `blocks` | header (`hash`, `height`, `bits`, `difficulty`, `chainwork`, `mediantime`...), coinbase script, reward/fee totals, input/output counts |
| `transactions` | txid (PK), wtxid `hash`, sizes/weight, version, locktime, `is_coinbase`, raw `hex`, output value |
| `inputs` | previous outpoint, sequence, scriptSig asm/hex, coinbase, witness |
| `outputs` | value, scriptPubKey asm/hex/type/address(es), `req_sigs` |
