# `Bitcoin` Raw Blockchain Data

> Cosmos
> [`sf.bitcoin.type.v1.Block`](https://buf.build/streamingfast/firehose-bitcoin/docs/main:sf.bitcoin.type.v1)

- [x] **Blocks**
- [x] **Transactions**
- [x] **Inputs**
- [x] **Outputs**

## Graph

```mermaid
graph TD;
  map_events[map: map_events];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> map_events;
  sf.bitcoin.type.v1.Block[source: sf.bitcoin.type.v1.Block] --> map_events;
```

## Modules

```bash
Name: map_events
Initial block: 0
Kind: map
Input: source: sf.substreams.v1.Clock
Input: source: sf.bitcoin.type.v1.Block
Output Type: proto:pinax.bitcoin.Events
Hash: 687432b625d3409cde6ac6e20c642554b84770ff
```