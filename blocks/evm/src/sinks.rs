use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_ethereum::pb::eth::v2::Block;

use crate::balance_changes::insert_balance_change;
use crate::blocks::insert_blocks;
use crate::transactions::insert_transactions;

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
    let mut tables: DatabaseChanges = DatabaseChanges::default();
    // blocks
    insert_blocks(&mut tables, &clock, &block);
    for balance_change in block.balance_changes.clone() {
        insert_balance_change(&mut tables, &clock, &balance_change);
    }
    // transactions
    insert_transactions(&mut tables, &clock, &block);
    Ok(tables)
}
