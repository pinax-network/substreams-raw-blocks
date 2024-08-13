## File size estimates

| chain                         | size (`2024-08-03`)  | ratios | estimated size | yearly growth | start date    | since start |
|-------------------------------|----------------------|--------|----------------|---------------|---------------|-------------|
| **ETH**                       | ~2.4 GB              | 1.00   | 7.93 TB        | 0.88 TB       | 2015-07-30    | 3292 days   |
| **BASE**                      | ~13.1 GB             | 5.45   | 43.22 TB       | 4.78 TB       | 2023-06-15    | 415 days    |
| **BSC**                       | ~6.8 GB              | 2.83   | 22.44 TB       | 2.48 TB       | 2020-08-29    | 1435 days   |
| **Polygon**                   | ~8.2 GB              | 3.41   | 27.04 TB       | 2.99 TB       | 2020-05-30    | 1526 days   |
| **ArbOne**                    | ~6.2 GB              | 2.58   | 20.46 TB       | 2.26 TB       | 2021-05-28    | 1163 days   |
| **Mode**                      | ~625 MB              | 0.26   | 2.06 TB        | 0.23 TB       | 2023-11-16    | 261 days    |
| **Linea**                     | ~1.1 GB              | 0.45   | 3.57 TB        | 0.40 TB       | 2023-07-06    | 394 days    |
| **XAI**                       | ~7.6 GB              | 3.16   | 25.06 TB       | 2.77 TB       | 2024-01-04    | 212 days    |
| **Zora**                      | ~0.233 GB            | 0.97   | 7.69 TB        | 0.09 TB       | 2023-06-13    | 417 days    |
| **Sepolia** (ETH)             | ~4.5 GB              | 1.87   | 14.83 TB       | 1.64 TB       | 2021-10-23    | 1015 days   |
| **Holesky** (ETH)             | ~1.07 GB             | 0.44   | 3.49 TB        | 0.39 TB       | 2023-09-28    | 310 days    |
| **Arb Sepolia** (Arbitrum)    | ~1.18 GB             | 0.49   | 3.89 TB        | 0.43 TB       | 2023-08-22    | 347 days    |
| **Chapel** (BNB)              | ~0.387 GB            | 0.16   | 1.27 TB        | 0.14 TB       | 2020-07-09    | 1486 days   |
| **Amoy** (Polygon)            | ~113 MB              | 0.05   | 0.40 TB        | 0.04 TB       | 2023-11-19    | 258 days    |

## Clickhouse Storage Size

**Total Size by Table**
```sql
SELECT
    table,
    formatReadableSize(sum(data_compressed_bytes)) AS compressed_size,
    formatReadableSize(sum(data_uncompressed_bytes)) AS uncompressed_size,
    round(sum(data_uncompressed_bytes) / sum(data_compressed_bytes), 2) AS ratio
FROM system.parts
WHERE (database = 'eth') AND active
GROUP BY table
ORDER BY sum(data_compressed_bytes) DESC

┌─table─────────────────┬─compressed_size─┬─uncompressed_size─┬─ratio─┐
│ traces                │ 2.09 TiB        │ 8.68 TiB          │  4.16 │
│ transactions          │ 1.35 TiB        │ 2.66 TiB          │  1.97 │
│ gas_changes           │ 1.17 TiB        │ 10.54 TiB         │  9.02 │
│ balance_changes       │ 1.14 TiB        │ 5.14 TiB          │  4.51 │
│ storage_changes       │ 1.02 TiB        │ 5.47 TiB          │  5.38 │
│ nonce_changes         │ 578.01 GiB      │ 1.12 TiB          │  1.98 │
│ logs                  │ 506.83 GiB      │ 2.78 TiB          │  5.62 │
│ account_creations     │ 67.93 GiB       │ 150.90 GiB        │  2.22 │
│ code_changes          │ 18.99 GiB       │ 101.82 GiB        │  5.36 │
│ blocks                │ 17.98 GiB       │ 27.63 GiB         │  1.54 │
│ block_balance_changes │ 4.58 GiB        │ 14.86 GiB         │  3.24 │
│ cursors               │ 1.31 KiB        │ 1020.00 B         │  0.76 │
└───────────────────────┴─────────────────┴───────────────────┴───────┘
```

**Total Size**
```sql
SELECT
    formatReadableSize(sum(data_compressed_bytes)) AS compressed_size,
    formatReadableSize(sum(data_uncompressed_bytes)) AS uncompressed_size,
    round(sum(data_uncompressed_bytes) / sum(data_compressed_bytes), 2) AS ratio
FROM system.parts
WHERE (database = 'eth') AND active

┌─compressed_size─┬─uncompressed_size─┬─ratio─┐
│ 7.93 TiB        │ 36.68 TiB         │  4.63 │
└─────────────────┴───────────────────┴───────┘
```
