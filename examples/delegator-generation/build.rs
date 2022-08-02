use near_sdk_abi::{AbiFile, Generator};

fn main() -> anyhow::Result<()> {
    Generator::new("gen".into())
        .file(AbiFile::new("src/adder.json"))
        .generate()?;
    Ok(())
}
