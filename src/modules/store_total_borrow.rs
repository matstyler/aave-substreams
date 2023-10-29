use substreams::store::StoreSetBigInt;
use crate::pb::aave::Transfers;

use crate::calls::get_total_borrow;

use substreams::store::StoreNew;
use substreams::store::StoreSet;

#[substreams::handlers::store]
pub fn store_total_borrow(borrow_transfers: Transfers, store: StoreSetBigInt) {
    if borrow_transfers.transfers.len() > 0 {
        store.set(1, "total_borrow", &get_total_borrow());
    }
}