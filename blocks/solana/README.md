## `Solana` Raw Blockchain Data

> Solana
> [`sf.solana.type.v1.Block`](https://buf.build/streamingfast/firehose-solana/docs/main:sf.solana.type.v1)

- [x] **Blocks**
- [x] **Transactions**
- [x] **Instruction Calls**
- [x] **Account Activity**
- [x] **Rewards**
- [x] **Vote Transactions**
- [x] **Vote Instruction Calls**
- [x] **Vote Account Activity**
- ~~[ ] **Discriminators**~~


```mermaid
graph TD;
  raw[sf.solana.type.v1.Block];
  raw --> blocks;
  raw --> transactions;
  raw --> instruction_calls;
  raw --> account_activity;
  raw --> rewards;
  raw --> vote_transactions;
  raw --> vote_instruction_calls;
  raw --> vote_account_activity;
  raw --> discriminators;
```