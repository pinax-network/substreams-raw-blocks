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
```
