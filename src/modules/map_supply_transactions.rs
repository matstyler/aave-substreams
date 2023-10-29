use substreams::Hex;
use substreams_ethereum::block_view::LogView;
use substreams_ethereum::pb::eth;

use crate::abi::pool;
use crate::pb::aave::{Transfer, Transfers, TransferType};

use crate::constants::TRACKED_POOL;
use crate::utils::{format_bigint_u64, format_hex};

pub fn format_supply_transfer(data: pool::events::Supply, log: LogView) -> Transfer {
    Transfer {
        id: format_hex(&log.receipt.transaction.hash),
        value: format_bigint_u64(data.amount),
        event: TransferType::Supply as i32,
        pool_address: Hex(TRACKED_POOL).to_string(),
    }
}

#[substreams::handlers::map]
fn map_supply_transactions(block: eth::v2::Block) -> Result<Transfers, substreams::errors::Error> {
    let supply_transfers: Vec<Transfer> = block
        .events::<pool::events::Supply>(&[&TRACKED_POOL])
        .map(|(data, log)| format_supply_transfer(data, log))
        .collect();

    Ok(Transfers { transfers: supply_transfers })
}