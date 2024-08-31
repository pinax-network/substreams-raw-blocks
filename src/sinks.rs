use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_antelope::pb::Block;
use substreams_entity_change::pb::entity::EntityChanges;

use crate::blocks::insert_blocks;

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
    let mut tables: DatabaseChanges = DatabaseChanges::default();

    // TABLE::blocks
    insert_blocks(&mut tables, &clock, &block);

    Ok(tables)
}

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();

    // blocks
    let block_hash = clock.id;
    let block_time = clock.timestamp.unwrap().seconds;
    let header = block.header.clone().unwrap_or_default();
    let previous = header.previous;

    // TABLE::blocks
    tables.create_row("blocks", &block_hash)
        .set("block_hash", &block_hash)
        .set("block_time", &block_time.to_string())
        .set("previous", &previous)
    ;

    // TABLE::transactions
    for transaction in block.transaction_traces() {

        // transactions
        let tx_hash = transaction.id.clone();

        tables.create_row("transactions", &tx_hash)
            .set("block_hash", &block_hash)
            .set("tx_hash", &tx_hash)
        ;

        // Traces of each action within the transaction, including all notified and nested actions.
        for trace in transaction.action_traces.iter() {
            // action
            let action = trace.action.clone().unwrap_or_default();
            let account = action.account;
            let name = action.name;
            let json_data = action.json_data;
            let raw_data = Hex::encode(&action.raw_data.to_vec());
            let action_ordinal = trace.action_ordinal;

            let key = format!("{}-{}", &tx_hash, &action_ordinal);
            tables.create_row("actions", &key)
                .set("tx_hash", &tx_hash)
                .set("action_ordinal", &action_ordinal.to_string())
                .set("account", &account)
                .set("name", &name)
                .set("json_data", &json_data)
                .set("raw_data", &raw_data)
            ;
        }
    }
    Ok(tables.to_entity_changes())
}
