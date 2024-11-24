CREATE OR REPLACE SECURE VIEW v1_transactions AS
SELECT
  -- block --
  TO_TIMESTAMP(block_time / 1000000000) as block_time,
  block_number,
  block_hash,
  block_date,

  -- block roots --
  transactions_root,
  receipts_root,

  -- transaction --
  index,
  hash,
  from,
  to,
  nonce,
  status,
  status_code,
  success,
  ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(gas_price_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as gas_price,
  gas_price_bytes,
  gas_limit,
  ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(value_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as value,
  value_bytes,
  data,
  v,
  r,
  s,
  gas_used,
  type,
  type_code,
  ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(max_fee_per_gas_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as max_fee_per_gas,
  max_fee_per_gas_bytes,
  ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(max_priority_fee_per_gas_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as max_priority_fee_per_gas,
  max_priority_fee_per_gas_bytes,
  begin_ordinal,
  end_ordinal,

  -- blobs --
  ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(blob_gas_price_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as blob_gas_price,
  blob_gas_price_bytes,
  blob_gas_used,
  ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(blob_gas_fee_cap_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as blob_gas_fee_cap,
  blob_gas_fee_cap_bytes,
  blob_gas,

  -- transaction receipt --
  cumulative_gas_used,
  logs_bloom,
  state_root
FROM "v1.0.0-transactions";

CREATE OR REPLACE SECURE VIEW v1_traces AS
SELECT
    -- block --
    TO_TIMESTAMP(block_time / 1000000000) as block_time,
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