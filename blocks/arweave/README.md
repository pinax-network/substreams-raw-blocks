# `Arweave` Raw Blockchain Data

> Cosmos
> [`sf.arweave.type.v1.Block`](https://buf.build/pinax/firehose-arweave/file/main:sf/arweave/type/v1/type.proto)

- [x] **Blocks**
- [x] **Transactions**
- [x] **Transaction Tags**

## Graph

```mermaid
graph TD;
  map_events[map: map_events];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> map_events;
  sf.arweave.type.v1.Block[source: sf.arweave.type.v1.Block] --> map_events;
```

## Modules

```bash

```
