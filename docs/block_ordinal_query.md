## Block Ordinal SQL Queries

Tables which consists of `ordinal` data can be used to query for specific transactions or traces. These tables are:
- `balance_changes`
- `storage_changes`
- `nonce_changes`
- `gas_changes`
- `code_changes`
- `account_creations`

**Example**

- **block_date**: `2023-06-25` (used to optimize the query)
- **block_number**: `17552799`
- **ordinal**: `9727`

**Lookup transactions**
```sql
SELECT
    hash,
    index
FROM transactions
WHERE (block_date = '2023-06-25') AND (block_number = 17552799) AND ((9727 >= begin_ordinal) AND (9727 <= end_ordinal))

┌─hash───────────────────────────────────────────────────────────────┬─index─┐
│ 0x123db611f89a2d781a2c22ed08b1ad3db480a964416a700c22d9f0c5b93d46bd │   367 │
└────────────────────────────────────────────────────────────────────┴───────┘

1 row in set. Elapsed: 0.007 sec. Processed 47.36 thousand rows, 1.23 MB (6.45 million rows/s., 167.10 MB/s.)
```
> 2.7B transactions are stored in the database, and the query is optimized to return the result in milliseconds.

**Lookup traces**
```sql
SELECT
    tx_hash,
    tx_index,
    index,
    parent_index
FROM traces
WHERE (block_date = '2023-06-25') AND (block_number = 17552799) AND ((9727 >= begin_ordinal) AND (9727 <= end_ordinal))

┌─tx_hash────────────────────────────────────────────────────────────┬─tx_index─┬─index─┬─parent_index─┐
│ 0x123db611f89a2d781a2c22ed08b1ad3db480a964416a700c22d9f0c5b93d46bd │      367 │     1 │            0 │
└────────────────────────────────────────────────────────────────────┴──────────┴───────┴──────────────┘

1 row in set. Elapsed: 0.024 sec. Processed 190.87 thousand rows, 4.60 MB (7.80 million rows/s., 187.90 MB/s.)
```
> 12B traces are stored in the database, and the query is optimized to return the result in milliseconds.