use common::sinks::insert_timestamp;
use common::utils::bytes_to_hex;
use common::{keys::balance_changes_keys, utils::optional_bigint_to_string};
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::BalanceChange;

pub fn balance_change_reason_to_string(reason: i32) -> String {
    match reason {
        0 => "Unknown".to_string(),
        1 => "RewardMineUncle".to_string(),
        2 => "RewardMineBlock".to_string(),
        3 => "DaoRefundContract".to_string(),
        4 => "DaoAdjustBalance".to_string(),
        5 => "Transfer".to_string(),
        6 => "GenesisBalance".to_string(),
        7 => "GasBuy".to_string(),
        8 => "RewardTransactionFee".to_string(),
        14 => "RewardFeeReset".to_string(),
        9 => "GasRefund".to_string(),
        10 => "TouchAccount".to_string(),
        11 => "SuicideRefund".to_string(),
        13 => "SuicideWithdraw".to_string(),
        12 => "CallBalanceOverride".to_string(),
        // -- Used on chain(s) where some Ether burning happens
        15 => "Burn".to_string(),
        16 => "Withdrawal".to_string(),
        // -- Rewards for Blob processing on BNB chain added in Tycho hard-fork, refers to BNB documentation to check the timestamp at which it was activated.
        17 => "RewardBlobFee".to_string(),
        // -- Used on optimism chan
        18 => "IncreaseMint".to_string(),
        _ => "Unknown".to_string(),
    }
}

// https://github.com/streamingfast/firehose-ethereum/blob/1bcb32a8eb3e43347972b6b5c9b1fcc4a08c751e/proto/sf/ethereum/type/v2/type.proto#L658
pub fn insert_balance_change(tables: &mut DatabaseChanges, clock: &Clock, balance_change: &BalanceChange) {
    let address = bytes_to_hex(balance_change.address.clone());
    let new_value = optional_bigint_to_string(balance_change.new_value.clone());
    let old_value = optional_bigint_to_string(balance_change.old_value.clone());
    let ordinal = balance_change.ordinal.to_string();
    let reason_code = balance_change.reason;
    let reason = balance_change_reason_to_string(reason_code);
    let keys = balance_changes_keys(&clock, &ordinal);
    let row = tables
        .push_change_composite("balance_changes", keys, 0, table_change::Operation::Create)
        .change("address", ("", address.as_str()))
        .change("new_value", ("", new_value.as_str()))
        .change("old_value", ("", old_value.as_str()))
        .change("ordinal", ("", ordinal.as_str()))
        .change("reason", ("", reason.as_str()))
        .change("reason_code", ("", reason_code.to_string().as_str()));

    insert_timestamp(row, clock, false);
}