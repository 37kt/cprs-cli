use anyhow::Context;
use toml::Table;

pub(crate) fn parse_toml(toml_path: &str) -> anyhow::Result<Table> {
    let toml_str = std::fs::read_to_string(toml_path).context(format!(
        "toml ファイルの読み込みに失敗しました: {}",
        toml_path
    ))?;

    Ok(toml::from_str(&toml_str)?)
}
