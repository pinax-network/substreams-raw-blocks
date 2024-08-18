## Method ID

Method ID is the first 4 bytes of the Keccak-256 hash of the function signature.

## Signature database

https://www.4byte.directory

> Function calls in the Ethereum Virtual Machine are specified by the first four bytes of data sent with a transaction. These 4-byte signatures are defined as the first four bytes of the Keccak hash (SHA3) of the canonical representation of the function signature. The database also contains mappings for event signatures. Unlike function signatures, they are defined as 32-bytes of the Keccak hash (SHA3) of the canonical form of the event signature. Since this is a one-way operation, it is not possible to derive the human-readable representation of the function or event from the bytes signature. This database is meant to allow mapping those bytes signatures back to their human-readable versions.

## Keccak-256

https://emn178.github.io/online-tools/keccak_256.html

> This Keccak-256 online tool helps you calculate hash from string or binary. You can input UTF-8, UTF-16, Hex to Keccak-256.

**Transactions**

```sql
SELECT
    left(data, 10) AS method_id,
    count() AS total
FROM eth.transactions
WHERE data != ''
GROUP BY method_id
ORDER BY total DESC
LIMIT 10

┌─method_id──┬──total─┐
│ 0xa9059cbb │ 137257 │
│ 0x095ea7b3 │  53650 │
│ 0x3593564c │  39473 │
│ 0x12aa3caf │   7047 │
│ 0x9871efa4 │   6141 │
│ 0x5f575529 │   6129 │
│ 0x78e111f6 │   5743 │
│ 0xb6f9de95 │   5455 │
│ 0x791ac947 │   4294 │
│ 0x23b872dd │   4059 │
└────────────┴────────┘
```

**Traces**

```sql
SELECT
    left(input, 10) AS method_id,
    count() AS total
FROM eth.traces
WHERE input != '' AND method_id != '0x00000000'
GROUP BY method_id
ORDER BY total DESC
LIMIT 10

Query id: 9fafc925-a7f8-4463-8b4f-056717388750

┌─method_id──┬───total─┐
│ 0x70a08231 │ 1476488 │
│ 0xa9059cbb │ 1077554 │
│ 0x23b872dd │  340640 │
│ 0x022c0d9f │  187959 │
│ 0x0902f1ac │  176874 │
│ 0x095ea7b3 │  133393 │
│ 0x128acb08 │  108070 │
│ 0xfa461e33 │  107904 │
│ 0xd0e30db0 │   95526 │
│ 0x2e1a7d4d │   75176 │
└────────────┴─────────┘
```