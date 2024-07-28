# Substreams Raw Blocks

## Raw Data

- [x] **Blocks**
  - [ ] `block_rewards`
    - [ ] `static_block_reward`
    - [ ] `transaction_fees`
    - [ ] `burnt_fees`
  - [ ] `processed_beacon_chain_withdrawals`
  - [ ] `contract_internal_transactions`
- [x] **Logs**
- [ ] **Transactions**
- [ ] **Traces**
  - [ ] **Creation Traces**
- [ ] **Beacon**
- [x] **BalanceChanges**

## Supported Networks

- **no-EVM**
  - [ ] Bitcoin
  - [ ] Solana
  - [ ] Near
  - [ ] Cosmos Hub
  - [ ] StartNet
  - [ ] Aptos
  - [ ] SUI
  - [ ] TON
- **EVM**
  - [x] Ethereum
  - [x] Arbitrum
  - [x] BNB
  - [x] Polygon
  - [ ] Fantom
  - [ ] zkSync
  - [ ] [Gravity](https://gravity.xyz/)
  - [ ] [Avalanche-C](https://avax.network/)
  - [ ] [SEI](https://www.sei.io/)
  - [ ] [Harmony](https://www.harmony.one/)
  - [ ] [Aurora](https://aurora.dev/) (L2 on NEAR protocol)
  - [ ] [Injective](https://injective.com/) (L2 on Cosmos)
  - [ ] [Rootstock](https://rootstock.io/) (L2 on Bitcoin)
  - [ ] [IoTex](https://iotex.io/)
- **Optimistic Rollup (EVM)**
  - [ ] Optimism (OP)
  - [ ] opBNB
  - [ ] Blast
  - [ ] Base
  - [ ] Mode
  - [ ] Zora
  - [ ] Boba
  - [ ] BobaBNB
- **Cosmos ecosystem**
  - [ ] Osmosis (L1)
- **Polygon's Validium**
  - [ ] X Layer
- **ZK Rollup (EVM)**
  - [ ] Scroll
  - [ ] Linea
  - [ ] Fuse
  - [ ] [Astar zkEVM](https://astar.network/) (L2)
- **Polkadot's Parachain**
  - [ ] Moonbeam
  - [ ] Moonriver
  - [ ] [Astar](https://astar.network/) (L1)
- **Tezos Smart Rollups**
  - [ ] Etherlink

## SQL

```sql
CREATE TABLE IF NOT EXISTS blocks
(
    time                    DateTime,
    number                  UInt64,
    date                    Date,
    hash                    String,
    parent_hash             String,
    nonce                   UInt64,
    ommers_hash             String,
    logs_bloom              String,
    transactions_root       String,
    state_root              String,
    receipts_root           String,
    miner                   String,
    difficulty              Int64,
    total_difficulty        Decimal(38, 0),
    size                    String,
    mix_hash                String,
    extra_data              String,
    gas_limit               UInt64,
    gas_used                UInt64,
    blob_gas_used           UInt64,
    transaction_count       String,
    base_fee_per_gas        String,
    parent_beacon_root      String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (hash)
        ORDER BY (hash);
```

## Data Visualization

- Dune's spellbook
- Snowflake
- BigQuery
- Databricks
  <https://docs.databricks.com/en/connect/storage/amazon-s3.html>
- Clickhouse
- Postgres
- Amazon Redshift
  <https://aws.amazon.com/redshift/>

## Graph

```mermaid
graph TD;
  graph_out[map: graph_out];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> graph_out;
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> graph_out;
```

## Export to Parquet & CSV

```sql
SELECT *
FROM blocks
WHERE date='2015-07-31'
INTO OUTFILE 'eth_2015-07-31_blocks.parquet'
FORMAT Parquet
```

```sql
SELECT *
FROM blocks
WHERE date='2015-07-31'
INTO OUTFILE 'eth_2015-07-31_blocks.csv'
FORMAT CSVWithNames
```
