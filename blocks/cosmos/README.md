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
