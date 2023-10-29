use substreams::store::StoreSetBigInt;
use crate::pb::aave::Transfers;

use crate::calls::get_total_supply;

use substreams::store::StoreNew;
use substreams::store::StoreSet;

#[substreams::handlers::store]
pub fn store_total_supply(supply_transfers: Transfers, store: StoreSetBigInt) {
    if supply_transfers.transfers.len() > 0 {
        store.set(0, "total_supply", &get_total_supply());
    }
}