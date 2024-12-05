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

## Graph

```mermaid
graph TD;
  map_events[map: map_events];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> map_events;
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_events;
```

## Modules

```bash
Name: map_events
Initial block: 0
Kind: map
Input: source: sf.substreams.v1.Clock
Input: source: sf.ethereum.type.v2.Block
Output Type: proto:evm.Events
Hash: 700ee7417a6e24ceb49db71e62d030aed84b017d
```
