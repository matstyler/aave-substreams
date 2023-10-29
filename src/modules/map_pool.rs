use substreams::scalar::BigInt;
use crate::pb::aave::Pool;
use substreams::store::{StoreGetBigInt};

use crate::utils::format_bigint_u64;

use substreams::store::StoreGet;

#[substreams::handlers::map]
pub fn map_pool(total_supply: StoreGetBigInt, total_borrow: StoreGetBigInt) -> Result<Pool, substreams::errors::Error> {
    Ok(Pool {
        total_supply: format_bigint_u64(total_supply.get_last("total_supply").unwrap_or(BigInt::zero())),
        total_borrow: format_bigint_u64(total_borrow.get_last("total_borrow").unwrap_or(BigInt::zero())),
    })
}