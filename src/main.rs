use flavours_v2_proto::config::InnerConfig;
use std::fs::read_to_string;

fn main() -> Result<(), anyhow::Error> {
    let config: InnerConfig = toml::from_str(&read_to_string("./config.toml")?)?;
    println!("{config:#?}");
    Ok(())
}
