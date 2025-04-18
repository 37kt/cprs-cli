use std::collections::BTreeMap;

use serde::Deserialize;

mod internal {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct CargoToml {
        pub package: Package,
        pub bin: Vec<Bin>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Bin {
        pub name: String,
        pub path: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Package {
        #[serde(rename = "name")]
        pub contest: String,
        pub metadata: Metadata,
    }

    #[derive(Debug, Deserialize)]
    pub struct Metadata {
        #[serde(rename = "cargo-compete")]
        pub cargo_compete: CargoCompete,
    }

    #[derive(Debug, Deserialize)]
    pub struct CargoCompete {
        pub bin: BTreeMap<String, Problem>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Problem {
        pub alias: String,
        #[serde(rename = "problem")]
        pub url: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct CompeteToml {
        pub submit: Submit,
    }

    #[derive(Debug, Deserialize)]
    #[serde(tag = "kind")]
    pub enum Submit {
        #[serde(rename = "file")]
        File { path: String },
        #[serde(rename = "command")]
        Command { args: Vec<String> },
    }
}

#[derive(Debug)]
pub struct Problem {
    pub name: String,
    pub alias: String,
    pub url: String,
    pub path: String,
}

pub use internal::Submit;

#[derive(Debug)]
pub struct Config {
    pub contest: String,
    pub problems: Vec<Problem>,
    pub submit: Submit,
}

pub fn load_config(cargo_toml: &str, compete_toml: &str) -> anyhow::Result<Config> {
    let cargo_toml: internal::CargoToml = toml::from_str(cargo_toml)?;
    let compete_toml: internal::CompeteToml = toml::from_str(compete_toml)?;

    let internal::CargoToml { package, bin } = cargo_toml;
    let internal::Package { contest, metadata } = package;
    let problems = metadata
        .cargo_compete
        .bin
        .into_iter()
        .map(|(name, problem)| Problem {
            alias: problem.alias,
            url: problem.url,
            path: bin
                .iter()
                .find(|bin| bin.name == name)
                .unwrap()
                .path
                .clone(),
            name,
        })
        .collect();

    Ok(Config {
        contest,
        problems,
        submit: compete_toml.submit,
    })
}
