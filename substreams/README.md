# Antelope Transactions Substreams

> EOS, WAX, Telos, Ultra...
> [`sf.antelope.type.v1.Block`](https://buf.build/pinax/firehose-antelope/docs/main:sf.antelope.type.v1)

- [x] **Blocks**
- [x] **Transactions**
- [x] **Actions**
  - [x] **Authorization**
  - [x] **Receiver**

## Substreams Graph

```mermaid
graph TD;
  ch_out[map: ch_out];
  ch_out:params[params] --> ch_out;
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> ch_out;
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> ch_out;
```