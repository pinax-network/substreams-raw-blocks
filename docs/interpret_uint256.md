## With `duckdb`

<https://duckdb.org/>

```sql
SELECT
    value_hex,
    value_bytes,
    try_cast(value_hex as int64),
    try_cast('0x' || hex(value_bytes) as int64),
FROM './out/traces/0020500000-0020500001.parquet'
WHERE value_bytes != ''
LIMIT 10;
```

## With `cddb`

<https://clickhouse.com/chdb>

```python
import chdb

result = chdb.sql("""
SELECT
    value_hex,
    hex(value_bytes),
    reinterpretAsUInt256(reverse(value_bytes)),
    reinterpretAsUInt256(reverse(unhex(substring(value_hex,3))))
FROM './out/traces/`0020500000-0020500001.parquet`'
WHERE value_bytes != ''
LIMIT 10;
""")
print(result)
```

## With `Snowflake`

<https://www.snowflake.com/>

```sql
SELECT
  value_hex,
  value_bytes,
  TRY_TO_DECIMAL(substring(value_hex, 3), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX'),
  TRY_TO_DECIMAL(hex_encode(value_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX'),
FROM traces;
```
