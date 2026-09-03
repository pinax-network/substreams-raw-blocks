# `Starknet` Raw Blockchain Data

> Starknet, served as [`sf.starknet.type.v1.Block`](https://github.com/streamingfast/firehose-starknet).

Output is shaped for [`substreams sink postgres`](https://docs.substreams.dev/how-to-guides/sinks/sql/relational-mappings):
every repeated field of `pinax.starknet.v2.Events` is a table, following the conventions of the other
packages in this repo (canonical identity columns, `bytes` felts, `NUMERIC(78,0)` amounts).

```bash
export SUBSTREAMS_API_KEY=...
export DSN="postgres://user:pass@localhost:5432/raw_blocks_starknet?sslmode=disable"
make build && make sink-setup ARGS="--bytes-encoding 0xhex" && make sink NETWORK=starknet ARGS="--bytes-encoding 0xhex"
```

## Tables

| Table | Source |
|---|---|
| `blocks` | header, roots, sequencer, L1 gas/data-gas prices (fri/wei), DA mode, version, counters |
| `transactions` | every transaction version (invoke v0/v1/v3, declare v0-v3, deploy, deploy account, L1 handler) with receipt: status, fee, execution resources, calldata, signature, resource bounds, DA modes |
| `events` | emitted events (`from_address`, `keys`, `data`) |
| `messages_sent` | L2 -> L1 messages |
| `storage_diffs`, `declared_classes`, `deployed_contracts`, `replaced_classes`, `nonce_diffs` | the block's state update, one row per entry |
