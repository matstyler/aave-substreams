use substreams::Hex;
use substreams_ethereum::block_view::LogView;
use substreams_ethereum::pb::eth;

use crate::abi::pool;
use crate::constants::TRACKED_POOL;

use crate::pb::aave::{Transfer, Transfers, TransferType};
use crate::utils::{format_bigint_u64, format_hex};

pub fn format_borrow_transfer(data: pool::events::Borrow, log: LogView) -> Transfer {
    Transfer {
        id: format_hex(&log.receipt.transaction.hash),
        value: format_bigint_u64(data.amount),
        event: TransferType::Borrow as i32,
        pool_address: Hex(TRACKED_POOL).to_string(),
    }
}

#[substreams::handlers::map]
fn map_borrow_transactions(block: eth::v2::Block) -> Result<Transfers, substreams::errors::Error> {
    let borrow_transfers: Vec<Transfer> = block
        .events::<pool::events::Borrow>(&[&TRACKED_POOL])
        .map(|(data, log)| format_borrow_transfer(data, log))
        .collect();

    Ok(Transfers { transfers: borrow_transfers })
}