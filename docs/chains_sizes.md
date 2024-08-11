## File size estimates

| chain             | size (`2024-08-03`) | start date    | days since start | notes |
|-------------------|---------------------|---------------|-------|
| **ETH**           | ~2.4 GB             | 2015-07-30    | 3292
| **BASE**          | ~13.1GB             | 2023-06-15    | 415
| **BSC**           | ~6.8GB              | 2020-08-29    | 1435
| **Polygon**       | ~8.2 GB             | 2020-05-30    | 1526
| **ArbOne**        | ~6.2GB              | 2021-05-28    | 1163
| **Mode**          | ~625MB              | 2023-11-16    | 261
| **Linea**         | ~1.1GB              | 2023-07-06    | 394
| **XAI**           | ~7.6GB              | 2024-01-04    | 212
| **Zora**          | ~233MB              | 2023-06-13    | 417
| **Sepolia**       | ~4.5GB              | 2021-10-23    | 1015   | ETH Testnet |
| **Holesky**       | ~4.5GB              | 2023-09-28    | 310    | ETH Testnet |
| **Arb Sepolia**   | ~1.18GB             | 2023-08-22    | 347    | Arbitrum Testnet |
| **Chapel**        | ~387.2MB            | 2020-07-09    | 1486   | BNB Testnet |
| **Amoy**          | ~113MB              | 2023-11-19    | 258    | Polygon Testnet |

## Clickhouse Storage Size

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

Query id: 19684368-d0c5-497e-b20f-a666295bd317

┌─table─────────────────┬─compressed_size─┬─uncompressed_size─┬─ratio─┐
│ balance_changes       │ 21.87 GiB       │ 120.82 GiB        │  5.52 │
│ traces                │ 21.78 GiB       │ 133.91 GiB        │  6.15 │
│ transactions          │ 18.72 GiB       │ 34.66 GiB         │  1.85 │
│ gas_changes           │ 14.42 GiB       │ 140.67 GiB        │  9.76 │
│ nonce_changes         │ 10.15 GiB       │ 23.47 GiB         │  2.31 │
│ storage_changes       │ 5.12 GiB        │ 28.49 GiB         │  5.57 │
│ logs                  │ 3.95 GiB        │ 19.65 GiB         │  4.98 │
│ blocks                │ 2.66 GiB        │ 7.58 GiB          │  2.85 │
│ account_creations     │ 2.37 GiB        │ 10.57 GiB         │  4.46 │
│ block_balance_changes │ 670.31 MiB      │ 1.26 GiB          │  1.93 │
│ code_changes          │ 469.11 MiB      │ 3.62 GiB          │  7.91 │
│ cursors               │ 894.00 B        │ 680.00 B          │  0.76 │
└───────────────────────┴─────────────────┴───────────────────┴───────┘

12 rows in set. Elapsed: 1.580 sec.
```

## Parquet File Sizes

**ETH** (~2.4 GB/daily)
```
449M    eth_balance_changes.parquet
5.1M    eth_block_balance_changes.parquet
7.3M    eth_blocks.parquet
512M    eth_storage_changes.parquet
976M    eth_traces.parquet
483M    eth_transactions.parquet
```
**BASE** (~13.1GB/daily)
```
1.4G    base_balance_changes.parquet
4.0K    base_block_balance_changes.parquet
 36M    base_blocks.parquet
2.7G    base_storage_changes.parquet
6.7G    base_traces.parquet
2.3G    base_transactions.parquet
```
**BSC** (~6.8GB/daily)
```
897M    bsc_balance_changes.parquet
4.2M    bsc_block_balance_changes.parquet
 34M    bsc_blocks.parquet
2.0G    bsc_storage_changes.parquet
2.6G    bsc_traces.parquet
1.3G    bsc_transactions.parquet
```
**Polygon** (~8.2 GB/daily)
```
1.1G    polygon_balance_changes.parquet
4.0K    polygon_block_balance_changes.parquet
 43M    polygon_blocks.parquet
1.8G    polygon_storage_changes.parquet
3.5G    polygon_traces.parquet
1.8G    polygon_transactions.parquet
```
**ArbOne** (~6.2GB/daily)
```
752M    arbone_balance_changes.parquet
4.0K    arbone_block_balance_changes.parquet
166M    arbone_blocks.parquet
1.7G    arbone_storage_changes.parquet
2.7G    arbone_traces.parquet
895M    arbone_transactions.parquet
```
**Mode** (~625MB/daily)
```
 46M    mode_balance_changes.parquet
4.0K    mode_block_balance_changes.parquet
 17M    mode_blocks.parquet
172K    mode_code_changes.parquet
112M    mode_storage_changes.parquet
401M    mode_traces.parquet
 49M    mode_transactions.parquet
```

**Linea** (~1.1GB /daily)
```
166M    linea_balance_changes.parquet
 29M    linea_blocks.parquet
360K    linea_code_changes.parquet
288M    linea_storage_changes.parquet
450M    linea_traces.parquet
241M    linea_transactions.parquet
```
**Sepolia** (4.5GB /daily) ETH Testnet
```
181M    sepolia_balance_changes.parquet
3.1M    sepolia_block_balance_changes.parquet
6.2M    sepolia_blocks.parquet
 24M    sepolia_code_changes.parquet
193M    sepolia_storage_changes.parquet
2.4G    sepolia_traces.parquet
1.7G    sepolia_transactions.parquet
```
**Arb Sepolia** (1.18GB/daily) Arbitrum Testnet
```
 82M    arbsepolia_balance_changes.parquet
 78M    arbsepolia_blocks.parquet
8.8M    arbsepolia_code_changes.parquet
241M    arbsepolia_storage_changes.parquet
561M    arbsepolia_traces.parquet
214M    arbsepolia_transactions.parquet
```
**Chapel** (387.2MB/daily) BNB Testnet
```
 62M    chapel_balance_changes.parquet
3.5M    chapel_block_balance_changes.parquet
 25M    chapel_blocks.parquet
5.7M    chapel_code_changes.parquet
 80M    chapel_storage_changes.parquet
114M    chapel_traces.parquet
 97M    chapel_transactions.parquet
```
**XAI** (7.6GB/daily)
```
1.7G    xai_balance_changes.parquet
165M    xai_blocks.parquet
180K    xai_code_changes.parquet
2.5G    xai_storage_changes.parquet
1.1G    xai_traces.parquet
2.2G    xai_transactions.parquet
```
**Zora** (233MB/daily)
```
 35M    zora_balance_changes.parquet
 19M    zora_blocks.parquet
848K    zora_code_changes.parquet
 59M    zora_storage_changes.parquet
 81M    zora_traces.parquet
 39M    zora_transactions.parquet
```

**Amoy** (113MB/daily) Polygon Testnet
```
 12M    amoy_balance_changes.parquet
 17M    amoy_blocks.parquet
2.6M    amoy_code_changes.parquet
 17M    amoy_storage_changes.parquet
 40M    amoy_traces.parquet
 25M    amoy_transactions.parquet
```
