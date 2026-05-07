use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RuntimePaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub log_dir: PathBuf,
}

pub fn default_paths() -> anyhow::Result<RuntimePaths> {
    let project_dirs = directories::ProjectDirs::from("id", "osource", "market")
        .ok_or_else(|| anyhow::anyhow!("unable to resolve project directories"))?;
    Ok(RuntimePaths {
        config_dir: project_dirs.config_dir().to_path_buf(),
        data_dir: project_dirs.data_dir().to_path_buf(),
        log_dir: project_dirs.cache_dir().join("logs"),
    })
}
