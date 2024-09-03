# Antelope Transactions Substreams

> EOS, WAX, Telos, Ultra...
> [`sf.antelope.type.v1.Block`](https://buf.build/pinax/firehose-antelope/docs/main:sf.antelope.type.v1)

- [x] **Blocks**
- [x] **Transactions**
  - [x] **DatabaseOperations**
- [x] **Actions**
  - [x] **Authorization**
  - [x] **Receiver**

## Support For

- [x] Clickhouse
- [x] Subgraph
- [ ] ~~Postgres~~

```mermaid
graph TD;
  ch_out[map: ch_out];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> ch_out;
  graph_out[map: graph_out];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> graph_out;
```
