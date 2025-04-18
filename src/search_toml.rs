use anyhow::Context;

pub(crate) fn search_toml_path(file_name: &str) -> anyhow::Result<String> {
    let mut current_dir = std::env::current_dir()?;

    loop {
        let cargo_toml = current_dir.join(file_name);
        if cargo_toml.exists() {
            return Ok(cargo_toml.to_string_lossy().to_string());
        }
        current_dir = current_dir
            .parent()
            .context(format!("{} が見つかりません", file_name))?
            .to_path_buf();
    }
}
