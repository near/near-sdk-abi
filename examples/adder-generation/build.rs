use near_sdk_abi::Config;

fn main() -> anyhow::Result<()> {
    let config = Config {
        out_dir: Some("gen".into()),
    };
    config.generate_ext(&[("src/adder.json", None)])?;
    Ok(())
}
