# `Beacon` Raw Blockchain Data

> Ethereum consensus layer, served as
> [`sf.beacon.type.v1.Block`](https://github.com/pinax-network/firehose-beacon) (Phase0 through Fusaka).

Output is shaped for [`substreams sink postgres`](https://docs.substreams.dev/how-to-guides/sinks/sql/relational-mappings):
every repeated field of `pinax.beacon.v2.Events` is a table. Naming follows
[pinax-network/firehose-parquet](https://github.com/pinax-network/firehose-parquet/tree/main/blocks/src/beacon),
extended with every additional field the Firehose block offers.

```bash
export SUBSTREAMS_API_KEY=...
export DSN="postgres://user:pass@localhost:5432/raw_blocks_beacon?sslmode=disable"
make build && make sink-setup && make sink NETWORK=eth-cl
```

## Tables

Every table starts with `block_num` (= slot), `block_id` (= root), `parent_num` (= parent slot), `parent_id`, `timestamp`, `date`.

| Table | Source |
|---|---|
| `blocks` | header, `spec`, signature, RANDAO reveal, graffiti, eth1 data, sync aggregate, KZG commitments, counters |
| `attestations` | aggregation bits, attestation data (slot, committee, roots, source/target), signature, Electra `committee_bits` |
| `deposits`, `voluntary_exits`, `proposer_slashings`, `attester_slashings` | consensus operations (with proofs, indices and signatures) |
| `execution_payload` | one row per block from Bellatrix on (execution block header fields, blob gas) |
| `blob_sidecars` | embedded blobs (Deneb+) with KZG commitment, proof and inclusion proof |
| `withdrawals` | execution-layer withdrawals (Capella+) |
| `bls_to_execution_changes` | BLS-to-execution credential changes (Capella+) |
| `execution_transactions` | raw execution-layer transactions carried by the payload |
| `deposit_requests`, `withdrawal_requests`, `consolidation_requests` | Electra execution requests |
