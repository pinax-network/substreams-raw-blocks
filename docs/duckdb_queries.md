## Connect to DuckDB

## Setup Credentials

```sql
CREATE OR REPLACE SECRET datasets(
    TYPE S3,
    KEY_ID '<PINAX_KEY_ID>',
    SECRET '<PINAX_SECRET>',
    REGION 'none',
    URL_STYLE 'path',
    ENDPOINT 'pub.store.eosnation.io'
);
```

## Glob Scan

**count blocks**

```sql
SELECT count(), max(number), min(number)
FROM read_parquet('s3://eth-datasets/blocks-2024-09-*.parquet');

┌──────────────┬─────────────┬─────────────┐
│ count_star() │ max(number) │ min(number) │
│    int64     │   uint64    │   uint64    │
├──────────────┼─────────────┼─────────────┤
│       109766 │    20866918 │    20651994 │
└──────────────┴─────────────┴─────────────┘
```

**count traces**

```sql
SELECT count(), max(block_number), min(block_number)
FROM read_parquet('s3://eth-datasets/traces-2024-09-*.parquet');

┌──────────────┬───────────────────┬───────────────────┐
│ count_star() │ max(block_number) │ min(block_number) │
│    int64     │      uint64       │      uint64       │
├──────────────┼───────────────────┼───────────────────┤
│    115286677 │          20866918 │          20651994 │
└──────────────┴───────────────────┴───────────────────┘
```

**group by and order by**

```sql
SELECT "from", count()
FROM read_parquet('s3://eth-datasets/traces-2024-09-*.parquet')
GROUP BY "from" ORDER BY count() DESC LIMIT 20;

┌────────────────────────────────────────────┬──────────────┐
│                    from                    │ count_star() │
│                    blob                    │    int64     │
├────────────────────────────────────────────┼──────────────┤
│ 0xae2fc483527b8ef99eb5d9b44875f005ba1fae13 │      1343506 │
│ 0x7830c87c02e56aff27fa8ab1241711331fa86f43 │       715200 │
│ 0x000000629fbcf27a347d1aeba658435230d74a5f │       643269 │
│ 0xe75ed6f453c602bd696ce27af11565edc9b46b0d │       517104 │
│ 0xfc9928f6590d853752824b0b403a6ae36785e535 │       392777 │
│ 0x000000633b68f5d8d3a86593ebb815b4663bcbe0 │       367000 │
│ 0x454ef2f69f91527856e06659f92a66f464c1ca4e │       354403 │
│ 0xf2099c4783921f44ac988b67e743daefd4a00efd │       338343 │
│ 0xd1fa51f2db23a9fa9d7bb8437b89fb2e70c60cb7 │       289726 │
│ 0x008300082c3000009e63680088f8c7f4d3ff2e87 │       287767 │
│ 0xb1b2d032aa2f52347fbcfd08e5c3cc55216e8404 │       216509 │
│ 0xc7899ff6a3ac2ff59261bd960a8c880df06e1041 │       213358 │
│ 0xe93685f3bba03016f02bd1828badd6195988d950 │       199149 │
├────────────────────────────────────────────┴──────────────┤
│ 20 rows                                         2 columns │
└───────────────────────────────────────────────────────────┘
```
