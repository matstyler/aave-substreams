use substreams::scalar::BigInt;

use substreams::Hex;

pub fn format_hex(address: &[u8]) -> String {
    format!("0x{}", Hex(address).to_string())
}

pub fn format_bigint_u64(value: BigInt) -> u64 {
    u64::try_from(value).unwrap_or(0)
}

pub fn get_transfer_type(test: i32) -> String {
    match test {
        0 => String::from("SUPPLY"),
        1 => String::from("BORROW"),
        _ => String::from("SUPPLY"),
    }
}