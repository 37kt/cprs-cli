use std::collections::BTreeMap;

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
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
    bin: Bin,
}

#[derive(Debug, Deserialize)]
struct Bin {
    problems: BTreeMap<String, Problem>,
}

#[derive(Debug, Deserialize)]
struct Problem {
    alias: String,
    #[serde(rename = "problem")]
    url: String,
}

pub fn load_config(cargo_toml: &str) -> anyhow::Result<CargoCompete> {
    let cargo_toml: CargoToml = toml::from_str(cargo_toml)?;
    Ok(cargo_toml.package.metadata.cargo_compete)
}
