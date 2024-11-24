import chdb

result = chdb.sql("""
SELECT
    value_hex,
    hex(value_bytes),
    reinterpretAsUInt256(reverse(value_bytes)),
    reinterpretAsUInt256(reverse(unhex(substring(value_hex,3))))
FROM './out/traces/0020500000-0020500001.parquet'
WHERE value_bytes != ''
LIMIT 10;
""")
print(result)