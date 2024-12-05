CREATE OR REPLACE SECURE VIEW v1_blocks AS
SELECT
  -- clock --
  time,
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

CREATE OR REPLACE SECURE VIEW v1_balance_changes AS
SELECT
    -- block --
    block_time,
    block_number,
    block_hash,
    block_date,

    -- transaction --
    tx_hash,

    -- balance change --
    address,
    ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(new_balance_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as new_balance,
    new_balance_bytes,
    ZEROIFNULL(TRY_TO_DECIMAL(hex_encode(old_balance_bytes), 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX')) as old_balance,
    old_balance_bytes,
    ordinal,
    reason,
    reason_code,
FROM "v1.0.0-balance_changes";

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

create or replace TABLE PINAX_DATASETS.ETHEREUM."v1.0.0-transactions" (
	BLOCK_TIME NUMBER(38,0),
	BLOCK_NUMBER NUMBER(38,0),
	BLOCK_HASH VARCHAR(16777216),
	BLOCK_DATE DATE,
	TRANSACTIONS_ROOT VARCHAR(16777216),
	RECEIPTS_ROOT VARCHAR(16777216),
	INDEX NUMBER(38,0),
	HASH VARCHAR(16777216),
	"from" VARCHAR(16777216),
	"to" VARCHAR(16777216),
	NONCE NUMBER(38,0),
	STATUS VARCHAR(16777216),
	STATUS_CODE NUMBER(38,0),
	SUCCESS BOOLEAN,
	GAS_PRICE_HEX VARCHAR(16777216),
	GAS_LIMIT NUMBER(38,0),
	VALUE_HEX VARCHAR(16777216),
	DATA VARCHAR(16777216),
	V VARCHAR(16777216),
	R VARCHAR(16777216),
	S VARCHAR(16777216),
	GAS_USED NUMBER(38,0),
	TYPE VARCHAR(16777216),
	TYPE_CODE NUMBER(38,0),
	MAX_FEE_PER_GAS_HEX VARCHAR(16777216),
	MAX_PRIORITY_FEE_PER_GAS_HEX VARCHAR(16777216),
	BEGIN_ORDINAL NUMBER(38,0),
	END_ORDINAL NUMBER(38,0),
	BLOB_GAS_PRICE_HEX VARCHAR(16777216),
	BLOB_GAS_USED NUMBER(38,0),
	BLOB_GAS_FEE_CAP_HEX VARCHAR(16777216),
	BLOB_HASHES ARRAY,
	CUMULATIVE_GAS_USED NUMBER(38,0),
	LOGS_BLOOM VARCHAR(16777216),
	STATE_ROOT VARCHAR(16777216)
);