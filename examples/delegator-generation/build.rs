use near_sdk_abi::Generator;

fn main() -> anyhow::Result<()> {
    Generator::new("gen".into())
        .file("src/adder.json")
        .generate()?;
    Ok(())
}
