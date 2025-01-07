use std::path::PathBuf;

use ambrosia::MealieApi;
use clap::Parser;

#[derive(Parser, Debug)]
struct Opt {
    config_file: PathBuf,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Config {
    mealie: MealieConfig,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct MealieConfig {
    address: String,
    api_token: String,
}

fn main() {
    let opts = Opt::parse();
    let config: Config = toml::from_str(
        &std::fs::read_to_string(opts.config_file).expect("failed to read config file"),
    )
    .expect("failed to deserialize config file");

    let api = MealieApi::new(&config.mealie.address, config.mealie.api_token.as_bytes());

    println!("{:?}", api.get_recipes().expect("recipe fetch error"));
}
