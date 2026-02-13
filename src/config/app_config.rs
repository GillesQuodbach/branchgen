use std::path::PathBuf;
use directories::ProjectDirs;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub team: String,
}

// Ou est le dosssier ?
pub fn get_config_dir() -> Result<PathBuf, String> {
    let proj_dir = ProjectDirs::from("fr","aps","branchgen-cli")
    .ok_or("Unable to find config dir")?;

    let config_path = proj_dir.config_dir();
    println!("{:?}", config_path);
    fs::create_dir_all(config_path).map_err(|e| format!("Unable to create config dir: {}", e))?;
    Ok(config_path.to_path_buf())
}

// quel est le chemin du fichier ?
pub fn get_config_file_path() -> Result<PathBuf, String> {
    let config_file = get_config_dir()?.join("config.json");
    Ok(config_file)
}

// est-ce qu'il existe ?
pub fn config_file_exists() -> Result<bool, String> {
    let config_file = get_config_file_path()?;
    config_file.try_exists().map_err(|e| format!("Check your configuration file: {}", e))
}

// creation/ecriture du fichier
pub fn create_config_file(team: String) -> Result<AppConfig, String> {
    let config_file_path = get_config_file_path()?;

    let config = AppConfig { team };

    let json = serde_json::to_string_pretty(&config).map_err(|e| format!("Unable to serialize config: {}", e))?;

    std::fs::write(config_file_path, json).map_err(|e| format!("Unable to write config file: {}", e))?;

    Ok(config)
}