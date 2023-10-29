use crate::pb::aave::Transfers;

#[substreams::handlers::map]
fn map_events(supply_transfers: Transfers, borrow_transfers: Transfers) -> Result<Transfers, substreams::errors::Error> {
    Ok(Transfers { transfers: [supply_transfers.transfers, borrow_transfers.transfers].concat() })
}