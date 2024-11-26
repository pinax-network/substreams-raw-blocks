CREATE OR REPLACE SECURE VIEW v1_blocks AS
SELECT
  -- clock --
  TO_TIMESTAMP(time / 1000000000) as time,
  number,
  date,
  hash,

  -- roots --
  ommers_hash,
  logs_bloom,
  transactions_root,
  state_root,
  receipts_root,
  withdrawals_root,
  parent_beacon_root,

  -- header --
  parent_hash,
  nonce,
  miner,
  difficulty,
  ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(total_difficulty_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as total_difficulty,
  mix_hash,
  extra_data,
  extra_data_utf8,
  gas_limit,
  gas_used,
  base_fee_per_gas,

  -- blobs --
  blob_gas_used,
  excess_blob_gas,
  ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(blob_gas_price_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as blob_gas_price,
  (IFF(blob_transactions[0] ='', [], blob_transactions)) as blob_transactions,
  (IFF(blob_hashes[0] ='', [], blob_hashes)) as blob_hashes,
  total_blob_transactions,
  total_blobs,

  -- counters --
  size,
  total_transactions,
  successful_transactions,
  failed_transactions,
  total_balance_changes,
  total_withdrawals,

  -- detail level --
  detail_level,
  detail_level_code,
FROM "v1.0.0-blocks";

CREATE OR REPLACE SECURE VIEW v1_transactions AS
SELECT
  -- block --
  block_time,
  block_number,
  block_hash,
  block_date,

  -- block roots --
  transactions_root,
  receipts_root,

  -- transaction --
  index,
  hash,
  "from",
  "to",
  nonce,
  status,
  status_code,
  success,
  ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(gas_price_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as gas_price,
  gas_limit,
  ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(value_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as value,
  data,
  v,
  r,
  s,
  gas_used,
  type,
  type_code,
  ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(max_fee_per_gas_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as max_fee_per_gas,
  ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(max_priority_fee_per_gas_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as max_priority_fee_per_gas,
  begin_ordinal,
  end_ordinal,

  -- blobs --
  ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(blob_gas_price_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as blob_gas_price,
  blob_gas_used,
  ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(blob_gas_fee_cap_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as blob_gas_fee_cap,
  (IFF(blob_hashes[0] ='', [], blob_hashes)) as blob_hashes,

  -- transaction receipt --
  cumulative_gas_used,
  logs_bloom,
  state_root
FROM "v1.0.0-transactions";

CREATE OR REPLACE SECURE VIEW v1_traces AS
SELECT
    -- block --
    block_time,
    block_number,
    block_hash,
    block_date,

    -- transaction --
    tx_hash,
    tx_index,
    tx_status,
    tx_status_code,
    tx_success,
    "from",
    "to",

    -- traces --
    index,
    parent_index,
    depth,
    caller,
    call_type,
    call_type_code,
    address,
    ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(value_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as value,
    value_bytes,
    gas_limit,
    gas_consumed,
    return_data,
    input,
    suicide,
    failure_reason,
    state_reverted,
    status_reverted,
    status_failed,
    executed_code,
    begin_ordinal,
    end_ordinal,
FROM "v1.0.0-traces";

CREATE OR REPLACE SECURE VIEW v1_logs AS
SELECT
  *
FROM "v1.0.0-logs";

CREATE OR REPLACE SECURE VIEW v1_account_creations AS
SELECT
  *
FROM "v1.0.0-account_creations";

CREATE OR REPLACE SECURE VIEW v1_storage_changes AS
SELECT
  *
FROM "v1.0.0-storage_changes";

CREATE OR REPLACE SECURE VIEW v1_code_changes AS
SELECT
  *
FROM "v1.0.0-code_changes";