## `EVM` Raw Blockchain Data
>
> Ethereum, Base, Arbitrum One, Polygon, BNB...
> [`sf.ethereum.type.v2.Block`](https://buf.build/streamingfast/firehose-ethereum/docs/main:sf.ethereum.type.v2)

- [x] **Blocks**
  - [x] **BalanceChanges** (EXTENDED)
  - [x] **Code Changes** (EXTENDED)
  - [x] **System Calls** (EXTENDED)
- [x] **Logs**
- [x] **Transactions**
- [x] **Traces** (EXTENDED)
  - [x] **BalanceChanges**
  - [x] **Storage Changes**
  - [x] **Code Changes**
  - [x] **Account Creation**
  - [x] **Gas Changes**
  - [x] **Nonce Changes**
  - [x] **Creation Traces**

## Graph

```mermaid
graph TD;
  raw[sf.ethereum.type.v2.Block];
  raw --> base(BASE)
  raw --> extended(EXTENDED);
  base --> blocks;
  base --> logs;
  base --> transactions;
  extended --> traces;
  extended --> balance_changes;
  extended --> storage_changes;
  extended --> code_changes;
  extended --> account_creations;
  extended --> gas_changes;
  extended --> nonce_changes;
  extended --> creation_traces;
```

## Modules

```bash
Name: map_events
Initial block: 0
Kind: map
Input: source: sf.substreams.v1.Clock
Input: source: sf.ethereum.type.v2.Block
Output Type: proto:evm.Events
Hash: e9a39ec8c7084a493d9a9f60c1f2f5d18f7505ad
```
