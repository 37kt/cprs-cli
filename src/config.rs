use std::collections::BTreeMap;

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CargoToml {
    package: Package,

    bin: Vec<Bin>,
}

#[derive(Debug, Deserialize)]
struct Bin {
    name: String,
    path: String,
}

#[derive(Debug, Deserialize)]
struct Package {
    metadata: Metadata,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    #[serde(rename = "cargo-compete")]
    cargo_compete: CargoCompete,
}

#[derive(Debug, Deserialize)]
pub struct CargoCompete {
    bin: BTreeMap<String, Problem>,
}

#[derive(Debug, Deserialize)]
struct Problem {
    alias: String,
    #[serde(rename = "problem")]
    url: String,
}

pub fn load_config(cargo_toml: &str) -> anyhow::Result<CargoToml> {
    let cargo_toml: CargoToml = toml::from_str(cargo_toml)?;
    Ok(cargo_toml)
}
