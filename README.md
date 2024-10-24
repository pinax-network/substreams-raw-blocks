# Substreams Raw Blocks

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

## `Antelope` Raw Blockchain Data

> EOS, WAX, Telos, Ultra...
> [`sf.antelope.type.v1.Block`](https://buf.build/pinax/firehose-antelope/docs/main:sf.antelope.type.v1)

- [x] **Blocks**
  - [ ] **Savanna Merkle Roots**
- [x] **Transactions**
  - [x] **Feature Operations**
  - [x] **Permission Operations**
    - [x] **Authority.Accounts**
    - [x] **Authority.Keys**
    - [x] **Authority.Waits**
  - [x] **RAM Operations**
  - [x] **Table Operations**
  - [x] **Creation Tree**
  - [ ] ~~**Deferred Transactions**~~
- [x] **Actions**
  - [x] **Authorization**
  - [x] **Auth Sequence**
  - [x] **Account RAM Deltas**
- [x] **Database Operations**

## Substreams Graph

```mermaid
graph TD;
  raw[sf.antelope.type.v1.Block];
  raw --> blocks;
  raw --> transactions;
  raw --> actions;
  raw --> db_ops;
```

## `Cosmos` Raw Blockchain Data

> Cosmos
> [`sf.cosmos.type.v2.Block`](https://buf.build/streamingfast/firehose-cosmos/docs/main:sf.cosmos.type.v2)

- [x] **Blocks**
- [x] **Transactions**
- [x] **Transaction Messages**
- [x] **Events**
  - [x] **Block Events**
  - [x] **Transaction Events**
- [x] **Misbehaviors**
- [x] **Validator Updates**
- [x] **Consensus Param Updates**

```mermaid
graph TD;
  raw[sf.cosmos.type.v2.Block];
    raw --> blocks;
    raw --> transactions;
    raw --> transaction_messages;
    raw --> events;
    raw --> misbehaviors;
    raw --> validator_updates;
    raw --> consensus_param_updates;
    
    events --> block_events;
    events --> transaction_events;
```

