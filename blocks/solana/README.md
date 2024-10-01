## `Solana` Raw Blockchain Data

> Solana
> [`sf.solana.type.v1.Block`](https://buf.build/streamingfast/firehose-solana/docs/main:sf.solana.type.v1)

- [ ] **Blocks**
- [ ] **Transactions**
- [ ] **Instruction Calls**
- [ ] **Account Activity**
- [ ] **Discriminators**
- [ ] **Rewards**
- [ ] **Vote Transactions**

```mermaid
graph TD;
  raw[sf.solana.type.v1.Block];
  raw --> blocks;
  raw --> transactions;
  raw --> instruction_calls;
  raw --> account_activity;
  raw --> discriminators;
  raw --> rewards;
  raw --> vote_transactions;
```