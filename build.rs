use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("Pool", "abi/Pool.json")?
        .generate()?
        .write_to_file("src/abi/pool.rs")?;

    Abigen::new("AToken", "abi/AToken.json")?
        .generate()?
        .write_to_file("src/abi/atoken.rs")?;

    Abigen::new("DebtToken", "abi/DebtToken.json")?
        .generate()?
        .write_to_file("src/abi/debt_token.rs")?;

    Ok(())
}
