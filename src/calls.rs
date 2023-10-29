use crate::abi;
use crate::constants::{AUSDC_ASSET, USDC_DEBT_ASSET};

use substreams::scalar::BigInt;

pub fn get_total_supply() -> BigInt {
    let total_supply = abi::atoken::functions::TotalSupply {};
    let total_supply_option = total_supply.call(AUSDC_ASSET.to_vec());

    total_supply_option.unwrap_or(BigInt::zero())
}

pub fn get_total_borrow() -> BigInt {
    let total_borrow = abi::debt_token::functions::TotalSupply {};
    let total_borrow_option = total_borrow.call(USDC_DEBT_ASSET.to_vec());

    total_borrow_option.unwrap_or(BigInt::zero())
}
