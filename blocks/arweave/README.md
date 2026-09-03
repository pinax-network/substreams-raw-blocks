# `Arweave` Raw Blockchain Data

> Arweave, served as [`sf.arweave.type.v1.Block`](https://github.com/streamingfast/firehose-arweave).

Output is shaped for [`substreams sink postgres`](https://docs.substreams.dev/how-to-guides/sinks/sql/relational-mappings):
every repeated field of `pinax.arweave.v2.Events` is a table, following the conventions of the other
packages in this repo (canonical identity columns, `bytes` hashes, `NUMERIC(78,0)` amounts).

```bash
export SUBSTREAMS_API_KEY=...
export DSN="postgres://user:pass@localhost:5432/raw_blocks_arweave?sslmode=disable"
make build && make sink-setup ARGS="--bytes-encoding base64" && make sink NETWORK=arweave ARGS="--bytes-encoding base64"
```

## Tables

| Table | Source |
|---|---|
| `blocks` | header (hashes, nonce, difficulty, retarget, weave/block size, reward pool, proof of access), counters |
| `transactions` | id (PK), format, owner, target, quantity, data payload + size + root, signature, reward |
| `transaction_tags` | tag name/value per transaction (UTF-8 text and raw bytes) |
| `block_tags` | block-level tags |
