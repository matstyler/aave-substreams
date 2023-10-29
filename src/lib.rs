mod pb;
mod abi;
mod calls;
mod constants;
mod utils;
mod modules;

use pb::aave::{Transfers};

use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use crate::pb::aave::{Pool};

use crate::utils::{get_transfer_type};
use crate::constants::TRACKED_POOL;

pub use modules::*;

#[substreams::handlers::map]
pub fn graph_out(transfers: Transfers, pool: Pool) -> Result<EntityChanges, substreams::errors::Error> {
    // hash map of name to a table
    let mut tables = Tables::new();

    for transfer in transfers.transfers.into_iter() {
        tables
            .create_row("Transfer", transfer.id.clone())
            .set("amount", transfer.value)
            .set("type", get_transfer_type(transfer.event))
            .set("pool", transfer.pool_address.clone());

        tables
            .update_row("Pool", Hex(TRACKED_POOL).to_string())
            .set("last_transfer", transfer.id.clone())
            .set("total_supply", pool.total_supply)
            .set("total_borrow", pool.total_borrow);
    }

    Ok(tables.to_entity_changes())
}
