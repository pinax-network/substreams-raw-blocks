## Export to Parquet & CSV

```sql
SELECT *
FROM blocks FINAL
WHERE date='2024-08-03'
INTO OUTFILE 'eth_2024-08-03_blocks.parquet'
FORMAT Parquet
```

```sql
SELECT *
FROM blocks FINAL
WHERE date='2024-08-03'
INTO OUTFILE 'eth_2024-08-03_blocks.csv'
FORMAT CSVWithNames
```
