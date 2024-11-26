use common::structs::BlockTimestamp;
use common::utils::bytes_to_hex;
use substreams_ethereum::pb::eth::v2::{BalanceChange, Block, TransactionTrace};

use crate::pb::pinax::evm::v1::BalanceChange as BalanceChangeEvent;

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
// DetailLevel: EXTENDED
pub fn collect_balance_changes(block: &Block, timestamp: &BlockTimestamp) -> Vec<BalanceChangeEvent> {
    let mut balance_changes: Vec<BalanceChangeEvent> = vec![];

    // Collect balance changes from system calls
    for call in &block.system_calls {
        for balance_change in &call.balance_changes {
            balance_changes.push(parse_balance_change(balance_change, &TransactionTrace::default(), timestamp));
        }
    }

    // Collect balance changes from transaction traces
    for transaction in &block.transaction_traces {
        for call in &transaction.calls {
            for balance_change in &call.balance_changes {
                balance_changes.push(parse_balance_change(balance_change, transaction, timestamp));
            }
        }
    }

    balance_changes
}

pub fn parse_balance_change(balance_change: &BalanceChange, transaction: &TransactionTrace, timestamp: &BlockTimestamp) -> BalanceChangeEvent {
    // let amount = optional_bigint_to_decimal(balance_change.new_value.clone()) - optional_bigint_to_decimal(balance_change.old_value.clone());
    BalanceChangeEvent {
        // block
        block_time: timestamp.time.to_string(),
        block_number: timestamp.number,
        block_hash: timestamp.hash.clone(),
        block_date: timestamp.date.clone(),

        // transaction
        tx_hash: Some(bytes_to_hex(&transaction.hash)),

        // balance changes
        address: bytes_to_hex(&balance_change.address),
        old_balance_bytes: balance_change.old_value.clone().unwrap_or_default().bytes,
        new_balance_bytes: balance_change.new_value.clone().unwrap_or_default().bytes,

        ordinal: balance_change.ordinal,
        reason: balance_change_reason_to_string(balance_change.reason),
        reason_code: balance_change.reason as u32,
    }
}
